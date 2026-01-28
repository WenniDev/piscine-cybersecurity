use nom::IResult;

use super::byte_order::ByteOrder;
use super::ifd::{IfdEntry, read_ifd_entry};
use super::value::Value;
use crate::image::parse_image_type;

pub struct ExifParser<'a> {
    byte_order: ByteOrder,
    tiff_header: &'a [u8],
}

impl<'a> ExifParser<'a> {
    pub fn is_little_endian(&self) -> bool {
        self.byte_order.0
    }

    pub fn new(file_bytes: &'a [u8]) -> Result<Self, String> {
        let (_, image_type) = parse_image_type(file_bytes)
            .map_err(|e| format!("Unrecognized image type: {:?}", e))?;

        let tiff_header = image_type
            .find_tiff_header(file_bytes)
            .map_err(|e| format!("EXIF data not found: {}", e))?;

        let (_, byte_order) =
            ByteOrder::new(tiff_header).map_err(|e| format!("Invalid byte order: {:?}", e))?;

        Ok(ExifParser {
            byte_order,
            tiff_header,
        })
    }

    pub fn resolve_value(&self, offset: u32, count: u32, field_type: u16) -> Option<Value> {
        let data = &self.tiff_header[offset as usize..];
        Value::from_bytes(data, count, field_type, &self.byte_order)
    }

    pub fn parse(&self) -> Result<Vec<Vec<IfdEntry>>, String> {
        let (_, (_, ifd_offset)) = parse_tiff_header(self.tiff_header)
            .map_err(|e| format!("Error parsing TIFF header: {:?}", e))?;

        let mut current_offset = Some(ifd_offset);
        let mut all_ifds = Vec::new();

        while let Some(offset) = current_offset {
            let (entries, next_offset) = self.parse_ifd(offset)?;

            for entry in &entries {
                if let Some(sub_offset) = entry.get_sub_ifd_offset() {
                    let (sub_entries, _) = self.parse_ifd(sub_offset)?;
                    all_ifds.push(sub_entries);
                }
            }

            all_ifds.push(entries);
            current_offset = next_offset;
        }

        Ok(all_ifds)
    }

    pub fn parse_ifd(&self, offset: u32) -> Result<(Vec<IfdEntry>, Option<u32>), String> {
        let data = &self.tiff_header[offset as usize..];

        let (mut data, num_entries) = self
            .byte_order
            .u16(data)
            .map_err(|e| format!("Error reading number of entries: {:?}", e))?;

        let mut entries = Vec::new();

        for _ in 0..num_entries {
            let (rest, mut entry) = read_ifd_entry(data, &self.byte_order)
                .map_err(|e| format!("Error reading IFD entry: {:?}", e))?;
            data = rest;

            if entry.offset.is_some() {
                entry.resolve(self);
            }

            entries.push(entry);
        }

        let (_, next_offset) = self
            .byte_order
            .u32(data)
            .map_err(|e| format!("Error reading next IFD offset: {:?}", e))?;

        let next = if next_offset == 0 {
            None
        } else {
            Some(next_offset)
        };

        Ok((entries, next))
    }
}

pub fn parse_byte_order(input: &[u8]) -> IResult<&[u8], ByteOrder> {
    use nom::bytes::complete::take;
    let (input, marker) = take(2usize)(input)?;
    let order = match marker {
        b"II" => ByteOrder(true),
        b"MM" => ByteOrder(false),
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    };
    Ok((input, order))
}

pub fn parse_tiff_header(input: &[u8]) -> IResult<&[u8], (ByteOrder, u32)> {
    let (input, byte_order) = parse_byte_order(input)?;
    let (input, magic) = byte_order.u16(input)?;
    if magic != 42 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, ifd_offset) = byte_order.u32(input)?;
    Ok((input, (byte_order, ifd_offset)))
}
