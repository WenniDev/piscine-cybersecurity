use super::byte_order::ByteOrder;

#[derive(Debug, Clone)]
pub enum Value {
    Byte(Vec<u8>),
    Ascii(String),
    Short(Vec<u16>),
    Long(Vec<u32>),
    Rational(Vec<(u32, u32)>),
    Unknown(Vec<u32>),
}

impl Value {
    pub fn type_size(field_type: u16) -> usize {
        match field_type {
            1 => 1, // Byte
            2 => 1, // Ascii
            3 => 2, // Short
            4 => 4, // Long
            5 => 8, // Rational
            _ => 0,
        }
    }

    pub fn from_inline(value: u32, count: u32, field_type: u16, byte_order: &ByteOrder) -> Self {
        match field_type {
            1 => {
                let bytes: Vec<u8> = (0..count)
                    .map(|i| {
                        if byte_order.0 {
                            ((value >> (i * 8)) & 0xFF) as u8
                        } else {
                            ((value >> (24 - i * 8)) & 0xFF) as u8
                        }
                    })
                    .collect();
                Value::Byte(bytes)
            }
            2 => {
                let bytes: Vec<u8> = (0..count)
                    .map(|i| {
                        if byte_order.0 {
                            ((value >> (i * 8)) & 0xFF) as u8
                        } else {
                            ((value >> (24 - i * 8)) & 0xFF) as u8
                        }
                    })
                    .collect();
                Value::Ascii(
                    String::from_utf8_lossy(&bytes)
                        .trim_end_matches('\0')
                        .to_string(),
                )
            }
            3 => {
                let shorts: Vec<u16> = (0..count)
                    .map(|i| {
                        if byte_order.0 {
                            ((value >> (i * 16)) & 0xFFFF) as u16
                        } else {
                            ((value >> (16 - i * 16)) & 0xFFFF) as u16
                        }
                    })
                    .collect();
                Value::Short(shorts)
            }
            4 => Value::Long(vec![value]),
            _ => Value::Unknown(vec![value]),
        }
    }

    pub fn from_bytes(
        data: &[u8],
        count: u32,
        field_type: u16,
        byte_order: &ByteOrder,
    ) -> Option<Self> {
        match field_type {
            1 => Some(Value::Byte(data[..count as usize].to_vec())),
            2 => {
                let s = std::str::from_utf8(&data[..count as usize])
                    .ok()?
                    .trim_end_matches('\0')
                    .to_string();
                Some(Value::Ascii(s))
            }
            3 => {
                let mut shorts = Vec::new();
                let mut input = data;
                for _ in 0..count {
                    let (rest, val) = byte_order.u16(input).ok()?;
                    shorts.push(val);
                    input = rest;
                }
                Some(Value::Short(shorts))
            }
            4 => {
                let mut longs = Vec::new();
                let mut input = data;
                for _ in 0..count {
                    let (rest, val) = byte_order.u32(input).ok()?;
                    longs.push(val);
                    input = rest;
                }
                Some(Value::Long(longs))
            }
            5 => {
                let mut rationals = Vec::new();
                let mut input = data;
                for _ in 0..count {
                    let (rest, num) = byte_order.u32(input).ok()?;
                    let (rest, den) = byte_order.u32(rest).ok()?;
                    rationals.push((num, den));
                    input = rest;
                }
                Some(Value::Rational(rationals))
            }
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Byte(v) if v.len() == 1 => write!(f, "{}", v[0]),
            Value::Byte(v) => write!(f, "{:?}", v),
            Value::Ascii(s) => write!(f, "{}", s),
            Value::Short(v) if v.len() == 1 => write!(f, "{}", v[0]),
            Value::Short(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Value::Long(v) if v.len() == 1 => write!(f, "{}", v[0]),
            Value::Long(v) => write!(
                f,
                "{}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Value::Rational(v) if v.len() == 1 => {
                let (num, den) = v[0];
                if den == 1 {
                    write!(f, "{}", num)
                } else {
                    write!(f, "{}/{}", num, den)
                }
            }
            Value::Rational(v) => {
                let rationals: Vec<String> = v
                    .iter()
                    .map(|(num, den)| {
                        if *den == 1 {
                            format!("{}", num)
                        } else {
                            format!("{}/{}", num, den)
                        }
                    })
                    .collect();
                write!(f, "{}", rationals.join(", "))
            }
            Value::Unknown(v) => write!(f, "{:?}", v),
        }
    }
}
