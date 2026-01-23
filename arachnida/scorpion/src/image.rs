use nom::{
    IResult,
    bytes::complete::{tag, take},
    number::complete::{be_u16, be_u32},
};

type Res<'a, T> = IResult<&'a [u8], T>;

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

/// Signatures magiques des formats supportés
mod magic {
    pub const JPEG: &[u8] = &[0xFF, 0xD8, 0xFF];
    pub const PNG: &[u8] = &[0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];
    pub const GIF87A: &[u8] = b"GIF87a";
    pub const GIF89A: &[u8] = b"GIF89a";
    pub const BMP: &[u8] = &[0x42, 0x4D];
}

#[derive(Debug, Clone, Copy)]
pub enum Image {
    Jpeg,
    Png,
    Gif,
    Bmp,
}

impl TryFrom<&[u8]> for Image {
    type Error = ();

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let checks: &[(&[u8], Image)] = &[
            (magic::JPEG, Image::Jpeg),
            (magic::PNG, Image::Png),
            (magic::GIF87A, Image::Gif),
            (magic::GIF89A, Image::Gif),
            (magic::BMP, Image::Bmp),
        ];

        checks
            .iter()
            .find(|(sig, _)| input.starts_with(sig))
            .map(|(_, img)| *img)
            .ok_or(())
    }
}

impl Image {
    pub fn find_exif(&self, input: &[u8]) -> Option<Vec<u8>> {
        match self {
            Image::Jpeg => jpeg::find_exif(input),
            Image::Png => png::find_exif(input),
            _ => None,
        }
    }
}

mod jpeg {
    use super::parse::*;

    const SOI: &[u8] = &[0xFF, 0xD8];
    const MARKER_PREFIX: &[u8] = &[0xFF];
    const EXIF_HEADER: &[u8] = b"Exif\0\0";

    pub fn find_exif(input: &[u8]) -> Option<Vec<u8>> {
        let (mut input, _) = tag_bytes(SOI)(input).ok()?;

        loop {
            let (rest, _) = tag_bytes(MARKER_PREFIX)(input).ok()?;
            let (rest, marker) = take_n(1)(rest).ok()?;

            match marker[0] {
                // APP1 - potentiellement EXIF
                0xE1 => {
                    let (rest, length) = u16_be(rest).ok()?;
                    let data_len = (length - 2) as usize;
                    let (_, data) = take_n(data_len)(rest).ok()?;

                    if data.starts_with(EXIF_HEADER) {
                        return Some(data[6..].to_vec());
                    }
                    input = &rest[data_len..];
                }
                // EOI ou SOS - fin de la recherche
                0xD9 | 0xDA => return None,
                // RST markers ou TEM - pas de longueur
                0xD0..=0xD7 | 0x01 => input = rest,
                // Autre segment avec longueur
                _ => {
                    let (rest, length) = u16_be(rest).ok()?;
                    input = &rest[(length - 2) as usize..];
                }
            }
        }
    }
}

mod png {
    use super::{magic, parse::*};

    pub fn find_exif(input: &[u8]) -> Option<Vec<u8>> {
        let (mut input, _) = tag_bytes(magic::PNG)(input).ok()?;

        loop {
            let (rest, length) = u32_be(input).ok()?;
            let (rest, chunk_type) = take_n(4)(rest).ok()?;
            let (rest, chunk_data) = take_n(length as usize)(rest).ok()?;
            let (rest, _crc) = take_n(4)(rest).ok()?;

            match chunk_type {
                b"eXIf" => return Some(chunk_data.to_vec()),
                b"IEND" => return None,
                _ => input = rest,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_jpeg() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(matches!(img, Image::Jpeg));
    }

    #[test]
    fn detect_png() {
        let data = [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n', 0x00];
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(matches!(img, Image::Png));
    }

    #[test]
    fn detect_gif87a() {
        let data = b"GIF87a...";
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(matches!(img, Image::Gif));
    }

    #[test]
    fn detect_gif89a() {
        let data = b"GIF89a...";
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(matches!(img, Image::Gif));
    }

    #[test]
    fn detect_bmp() {
        let data = [0x42, 0x4D, 0x00, 0x00];
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(matches!(img, Image::Bmp));
    }

    #[test]
    fn detect_unknown_format() {
        let data = [0x00, 0x01, 0x02, 0x03];
        assert!(Image::try_from(data.as_slice()).is_err());
    }

    #[test]
    fn detect_empty_input() {
        let data: &[u8] = &[];
        assert!(Image::try_from(data).is_err());
    }

    #[test]
    fn jpeg_with_exif() {
        let data = vec![
            0xFF, 0xD8, // SOI
            0xFF, 0xE1, // APP1 marker
            0x00, 0x10, // Length (16 bytes incluant les 2 bytes de longueur)
            b'E', b'x', b'i', b'f', 0x00, 0x00, // "Exif\0\0"
            b'T', b'I', b'F', b'F', b'D', b'A', b'T', b'A', // TIFF data (8 bytes)
            0xFF, 0xD9, // EOI
        ];

        let result = jpeg::find_exif(&data);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), b"TIFFDATA");
    }

    #[test]
    fn jpeg_without_exif() {
        let data = vec![
            0xFF, 0xD8, // SOI
            0xFF, 0xE0, // APP0 marker (JFIF)
            0x00, 0x10, // Length
            b'J', b'F', b'I', b'F', 0x00, // "JFIF\0"
            0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF,
            0xDA, // SOS - fin de la recherche
        ];

        let result = jpeg::find_exif(&data);
        assert!(result.is_none());
    }

    #[test]
    fn jpeg_app1_non_exif() {
        let data = vec![
            0xFF, 0xD8, // SOI
            0xFF, 0xE1, // APP1 marker
            0x00, 0x0C, // Length
            b'h', b't', b't', b'p', b':', b'/', b'/', b'x', b'm', b'p', // Not "Exif\0\0"
            0xFF, 0xD9, // EOI
        ];

        let result = jpeg::find_exif(&data);
        assert!(result.is_none());
    }

    #[test]
    fn jpeg_invalid_no_soi() {
        let data = vec![0x00, 0x00, 0xFF, 0xE1];
        let result = jpeg::find_exif(&data);
        assert!(result.is_none());
    }

    #[test]
    fn png_with_exif() {
        // PNG minimal avec chunk eXIf
        let mut data = vec![
            0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n', // PNG signature
        ];

        // IHDR chunk (minimal)
        let ihdr_data = [
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00,
        ];
        data.extend_from_slice(&(ihdr_data.len() as u32).to_be_bytes()); // length
        data.extend_from_slice(b"IHDR");
        data.extend_from_slice(&ihdr_data);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC (ignoré)

        // eXIf chunk
        let exif_data = b"TIFFDATA";
        data.extend_from_slice(&(exif_data.len() as u32).to_be_bytes());
        data.extend_from_slice(b"eXIf");
        data.extend_from_slice(exif_data);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

        // IEND chunk
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // length 0
        data.extend_from_slice(b"IEND");
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC

        let result = png::find_exif(&data);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), b"TIFFDATA");
    }

    #[test]
    fn png_without_exif() {
        let mut data = vec![0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];

        // IHDR chunk
        let ihdr_data = [
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00,
        ];
        data.extend_from_slice(&(ihdr_data.len() as u32).to_be_bytes());
        data.extend_from_slice(b"IHDR");
        data.extend_from_slice(&ihdr_data);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        // IEND chunk (pas de eXIf)
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(b"IEND");
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        let result = png::find_exif(&data);
        assert!(result.is_none());
    }

    #[test]
    fn png_invalid_signature() {
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let result = png::find_exif(&data);
        assert!(result.is_none());
    }

    #[test]
    fn gif_has_no_exif_support() {
        let data = b"GIF89a...";
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(img.find_exif(data).is_none());
    }

    #[test]
    fn bmp_has_no_exif_support() {
        let data = [0x42, 0x4D, 0x00, 0x00];
        let img = Image::try_from(data.as_slice()).unwrap();
        assert!(img.find_exif(&data).is_none());
    }
}
