use {ReadBuf, ReadError};

pub trait Decodable {
    fn decode<R>(bytes: &mut R) -> Result<Self, ReadError>
    where
        Self: Sized,
        R: ReadBuf;
}
