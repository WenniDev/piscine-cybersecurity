use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    Byte(Vec<u8>),
    Ascii(String),
    Short(Vec<u16>),
    Long(Vec<u32>),
    Rational(Vec<(u32, u32)>),
    Undefined(Vec<u8>),
    SRational(Vec<(i32, i32)>),
}

pub struct Data {
    pub entries: HashMap<u16, Value>,
    pub exif_ifs: HashMap<u16, Value>,
    pub gps_ifs: HashMap<u16, Value>,
}
