use nom::{
    IResult,
    bytes::complete::take,
    number::complete::{be_i32, be_u16, be_u32, le_i32, le_u16, le_u32},
};

use crate::image::{ImageType, parse_image_type};

enum Tags {
    ImageWidth,
    ImageLength,
    BitsPerSample,
    Compression,
    PhotometricInterpretation,
    Orientation,
    SamplesPerPixel,
    PlanarConfiguration,
    YCbCrSubSampling,
    YCbCrPositioning,
    XResolution,
    YResolution,
    ResolutionUnit,
    StripOffsets,
    RowsPerStrip,
    StripByteCounts,
    JPEGInterchangeFormat,
    JPEGInterchangeFormatLength,
    TransferFunction,
    WhitePoint,
    PrimaryChromaticities,
    YCbCrCoefficients,
    ReferenceBlackWhite,
    DateTime,
    ImageDescription,
    Make,
    Model,
    Software,
    Artist,
    Copyright,
    Unknown,
}

impl From<u16> for Tags {
    fn from(tag: u16) -> Self {
        match tag {
            0x100 => Tags::ImageWidth,
            0x101 => Tags::ImageLength,
            0x102 => Tags::BitsPerSample,
            0x103 => Tags::Compression,
            0x106 => Tags::PhotometricInterpretation,
            0x112 => Tags::Orientation,
            0x115 => Tags::SamplesPerPixel,
            0x11C => Tags::PlanarConfiguration,
            0x212 => Tags::YCbCrSubSampling,
            0x213 => Tags::YCbCrPositioning,
            0x11A => Tags::XResolution,
            0x11B => Tags::YResolution,
            0x128 => Tags::ResolutionUnit,
            0x111 => Tags::StripOffsets,
            0x116 => Tags::RowsPerStrip,
            0x117 => Tags::StripByteCounts,
            0x201 => Tags::JPEGInterchangeFormat,
            0x202 => Tags::JPEGInterchangeFormatLength,
            0x12D => Tags::TransferFunction,
            0x13E => Tags::WhitePoint,
            0x13F => Tags::PrimaryChromaticities,
            0x211 => Tags::YCbCrCoefficients,
            0x214 => Tags::ReferenceBlackWhite,
            0x132 => Tags::DateTime,
            0x10E => Tags::ImageDescription,
            0x10F => Tags::Make,
            0x110 => Tags::Model,
            0x131 => Tags::Software,
            0x13B => Tags::Artist,
            0x8298 => Tags::Copyright,
            _ => Tags::Unknown,
        }
    }
}

impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Tags::ImageWidth => "Image width",
            Tags::ImageLength => "Image height",
            Tags::BitsPerSample => "Number of bits per component",
            Tags::Compression => "Compression scheme",
            Tags::PhotometricInterpretation => "Pixel composition",
            Tags::Orientation => "Orientation of image",
            Tags::SamplesPerPixel => "Number of components",
            Tags::PlanarConfiguration => "Image data arrangement",
            Tags::YCbCrSubSampling => "Subsampling ratio of Y to C",
            Tags::YCbCrPositioning => "Y and C positioning",
            Tags::XResolution => "Image resolution in width direction",
            Tags::YResolution => "Image resolution in height direction",
            Tags::ResolutionUnit => "Unit of X and Y resolution",
            Tags::StripOffsets => "Image data location",
            Tags::RowsPerStrip => "Number of rows per strip",
            Tags::StripByteCounts => "Bytes per compressed strip",
            Tags::JPEGInterchangeFormat => "Offset to JPEG SOI",
            Tags::JPEGInterchangeFormatLength => "Bytes of JPEG data",
            Tags::TransferFunction => "Transfer function",
            Tags::WhitePoint => "White point chromaticity",
            Tags::PrimaryChromaticities => "Chromaticities of primaries",
            Tags::YCbCrCoefficients => "Color space transformation matrix coefficients",
            Tags::ReferenceBlackWhite => "Pair of black and white reference values",
            Tags::DateTime => "File change date and time",
            Tags::ImageDescription => "Image title",
            Tags::Make => "Image input equipment manufacturer",
            Tags::Model => "Image input equipment model",
            Tags::Software => "Software used",
            Tags::Artist => "Person who created the image",
            Tags::Copyright => "Copyright holder",
            Tags::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

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

struct IfdEntry {
    tag: Tags,
    field_type: u16,
    count: u32,
    value: Value,
}

pub struct ExifParser<'a> {
    byte_order: ByteOrder,
    image_type: ImageType,
    data: &'a [u8],
}

impl<'a> ExifParser<'a> {
    pub fn new(file_bytes: &'a [u8]) -> Self {
        let (data, image_type) = parse_image_type(file_bytes).unwrap();
        let data = image_type.find_tiff_header(data).unwrap();

        ExifParser {
            byte_order: ByteOrder::new(file_bytes),
            image_type,
            data,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ByteOrder(bool); // true = little endian

impl ByteOrder {
    fn new(input: &[u8]) -> Self {
        let (input, marker) =
            take::<usize, &[u8], nom::error::Error<&[u8]>>(2usize)(input).unwrap();
        match marker {
            b"II" => ByteOrder(true),
            b"MM" => ByteOrder(false),
            _ => panic!("Invalid byte order"),
        }
    }

    fn u16<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u16> {
        if self.0 { le_u16(input) } else { be_u16(input) }
    }

    fn u32<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u32> {
        if self.0 { le_u32(input) } else { be_u32(input) }
    }

    fn i32<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], i32> {
        if self.0 { le_i32(input) } else { be_i32(input) }
    }
}

pub fn parse_byte_order(input: &[u8]) -> IResult<&[u8], ByteOrder> {
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
    let (input, _) = byte_order.u16(input)?; // TIFF magic number
    let (input, ifd_offset) = byte_order.u32(input)?;
    Ok((input, (byte_order, ifd_offset)))
}

pub fn get_ifd_value(count: u32, type_size: usize) -> Value {
    todo!("To implement")
}

pub fn parse_ifd_entry<'a>(input: &'a [u8], byte_order: &ByteOrder) -> IResult<&'a [u8], IfdEntry> {
    let (input, tag) = byte_order.u16(input)?;
    let (input, field_type) = byte_order.u16(input)?;
    let (input, count) = byte_order.u32(input)?;
    let (input, value_offset) = byte_order.u32(input)?;

    let value = get_ifd_value(count, size_of_val(&field_type));

    Ok((
        input,
        IfdEntry {
            tag: tag.into(),
            field_type,
            count,
            value,
        },
    ))
}
