use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    combinator::value,
    number::complete::{be_u16, be_u32},
};

type Res<'a, T> = IResult<&'a [u8], T>;

#[derive(Debug)]
pub enum ExifError {
    InvalidFormat,
    ExifNotFound,
    NoStartOfImage,
    NoMarkerFound,
}

impl std::fmt::Display for ExifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExifError::InvalidFormat => write!(f, "Invalid EXIF format"),
            ExifError::ExifNotFound => write!(f, "EXIF data not found"),
            ExifError::NoStartOfImage => write!(f, "No Start of Image marker found"),
            ExifError::NoMarkerFound => write!(f, "No valid marker found in image"),
        }
    }
}

type Result<T> = std::result::Result<T, ExifError>;

mod parse {
    use super::*;

    pub fn tag_bytes<'a>(bytes: &'a [u8]) -> impl Fn(&'a [u8]) -> Res<'a, &'a [u8]> {
        move |input| tag(bytes)(input)
    }

    pub fn take_n(n: usize) -> impl Fn(&[u8]) -> Res<&[u8]> {
        move |input| take(n)(input)
    }

    pub fn u16_be(input: &[u8]) -> Res<'_, u16> {
        be_u16(input)
    }

    pub fn u32_be(input: &[u8]) -> Res<'_, u32> {
        be_u32(input)
    }
}

mod magic {
    pub const JPEG: &[u8] = &[0xFF, 0xD8, 0xFF];
    pub const PNG: &[u8] = &[0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];
    pub const GIF87A: &[u8] = b"GIF87a";
    pub const GIF89A: &[u8] = b"GIF89a";
    pub const BMP: &[u8] = &[0x42, 0x4D];
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Jpeg,
    Png,
    Gif,
    Bmp,
}

pub fn parse_image_type(input: &[u8]) -> IResult<&[u8], ImageType> {
    alt((
        value(ImageType::Jpeg, tag(magic::JPEG)),
        value(ImageType::Png, tag(magic::PNG)),
        value(ImageType::Gif, tag(magic::GIF87A)),
        value(ImageType::Gif, tag(magic::GIF89A)),
        value(ImageType::Bmp, tag(magic::BMP)),
    ))
    .parse(input)
}

impl ImageType {
    pub fn find_tiff_header(&self, input: &[u8]) -> Result<Vec<u8>> {
        match self {
            ImageType::Jpeg => jpeg::find_tiff_header(input),
            ImageType::Png => png::find_tiff_header(input),
            _ => Err(ExifError::InvalidFormat),
        }
    }
}

mod jpeg {
    use super::parse::*;
    use super::{ExifError, Result};

    const SOI: &[u8] = &[0xFF, 0xD8];
    const MARKER_PREFIX: &[u8] = &[0xFF];
    const EXIF_HEADER: &[u8] = b"Exif\0\0";

    pub fn find_tiff_header(input: &[u8]) -> Result<Vec<u8>> {
        let (mut input, _) = tag_bytes(SOI)(input)
            .ok()
            .ok_or(ExifError::NoStartOfImage)?;

        loop {
            let (rest, _) = tag_bytes(MARKER_PREFIX)(input)
                .ok()
                .ok_or(ExifError::NoMarkerFound)?;
            let (rest, marker) = take_n(1)(rest).ok().ok_or(ExifError::NoMarkerFound)?;

            match marker[0] {
                // APP1 - potentiellement EXIF
                0xE1 => {
                    let (rest, length) = u16_be(rest).ok().ok_or(ExifError::InvalidFormat)?;
                    let data_len = (length - 2) as usize;
                    let (_, data) = take_n(data_len)(rest)
                        .ok()
                        .ok_or(ExifError::InvalidFormat)?;

                    if data.starts_with(EXIF_HEADER) {
                        return Ok(data[6..].to_vec());
                    }
                    input = &rest[data_len..];
                }
                // EOI ou SOS - fin de la recherche
                0xD9 | 0xDA => return Err(ExifError::ExifNotFound),
                // RST markers ou TEM - pas de longueur
                0xD0..=0xD7 | 0x01 => input = rest,
                // Autre segment avec longueur
                _ => {
                    let (rest, length) = u16_be(rest).ok().ok_or(ExifError::InvalidFormat)?;
                    input = &rest[(length - 2) as usize..];
                }
            }
        }
    }
}

mod png {
    use super::{ExifError, Result};
    use super::{magic, parse::*};

    pub fn find_tiff_header(input: &[u8]) -> Result<Vec<u8>> {
        let (mut input, _) = tag_bytes(magic::PNG)(input)
            .ok()
            .ok_or(ExifError::InvalidFormat)?;

        loop {
            let (rest, length) = u32_be(input).ok().ok_or(ExifError::InvalidFormat)?;
            let (rest, chunk_type) = take_n(4)(rest).ok().ok_or(ExifError::InvalidFormat)?;
            let (rest, chunk_data) = take_n(length as usize)(rest)
                .ok()
                .ok_or(ExifError::InvalidFormat)?;
            let (rest, _crc) = take_n(4)(rest).ok().ok_or(ExifError::InvalidFormat)?;

            match chunk_type {
                b"eXIf" => return Ok(chunk_data.to_vec()),
                b"IEND" => return Err(ExifError::ExifNotFound),
                _ => input = rest,
            }
        }
    }
}
