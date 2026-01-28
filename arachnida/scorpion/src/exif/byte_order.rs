use nom::{
    IResult,
    bytes::complete::take,
    number::complete::{be_u16, be_u32, le_u16, le_u32},
};

#[derive(Debug, Clone, Copy)]
pub struct ByteOrder(pub bool); // true = little endian

impl ByteOrder {
    pub fn new(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, marker) = take::<usize, &[u8], nom::error::Error<&[u8]>>(2usize)(input)?;
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

    pub fn u16<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u16> {
        if self.0 { le_u16(input) } else { be_u16(input) }
    }

    pub fn u32<'a>(&self, input: &'a [u8]) -> IResult<&'a [u8], u32> {
        if self.0 { le_u32(input) } else { be_u32(input) }
    }
}
