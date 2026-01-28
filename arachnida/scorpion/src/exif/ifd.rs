use super::byte_order::ByteOrder;
use super::parser::ExifParser;
use super::value::Value;
use crate::tags::Tags;
use nom::IResult;

pub struct IfdEntry {
    pub tag: Tags,
    pub field_type: u16,
    pub count: u32,
    pub offset: Option<u32>,
    pub value: Option<Value>,
}

impl IfdEntry {
    pub fn resolve(&mut self, parser: &ExifParser) {
        if let Some(offset) = self.offset {
            self.value = parser.resolve_value(offset, self.count, self.field_type);
        }
    }

    pub fn get_sub_ifd_offset(&self) -> Option<u32> {
        if !self.tag.is_sub_ifd_pointer() {
            return None;
        }

        match &self.value {
            Some(Value::Long(v)) if v.len() == 1 => Some(v[0]),
            _ => None,
        }
    }
}

pub fn read_ifd_entry<'a>(input: &'a [u8], byte_order: &ByteOrder) -> IResult<&'a [u8], IfdEntry> {
    let (input, tag) = byte_order.u16(input)?;
    let (input, field_type) = byte_order.u16(input)?;
    let (input, count) = byte_order.u32(input)?;
    let (input, value_or_offset) = byte_order.u32(input)?;
    let data_size = count as usize * Value::type_size(field_type);

    let (value, offset) = if data_size > 4 {
        (None, Some(value_or_offset))
    } else {
        (
            Some(Value::from_inline(
                value_or_offset,
                count,
                field_type,
                byte_order,
            )),
            None,
        )
    };

    Ok((
        input,
        IfdEntry {
            tag: tag.into(),
            field_type,
            count,
            value,
            offset,
        },
    ))
}
