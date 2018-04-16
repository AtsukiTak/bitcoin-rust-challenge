pub trait Decodable {
    /// Decode `Self` from given bytes.
    fn decode<R: ReadBuffer>(buf: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub enum DecodeError {
    ShortBuffer,
    InvalidBytes,
}

pub trait ReadBuffer: Sized {
    fn read_bytes(&mut self, size: usize) -> Result<&[u8], DecodeError>;

    fn read<D: Decodable>(&mut self) -> Result<D, DecodeError> {
        D::decode(self)
    }
}

impl<'a, B> ReadBuffer for ::std::io::Cursor<&'a B>
where
    B: AsRef<[u8]>,
{
    fn read_bytes(&mut self, size: usize) -> Result<&[u8], DecodeError> {
        let start_pos = self.position() as usize; // Should I check here?

        if self.get_ref().as_ref().len() < start_pos + size {
            return Err(DecodeError::ShortBuffer);
        }
        self.set_position((start_pos + size) as u64);

        let buf = &self.get_ref().as_ref()[start_pos..];

        Ok(&buf[..size])
    }
}

impl<'a> ReadBuffer for ::std::io::Cursor<&'a [u8]>
{
    fn read_bytes(&mut self, size: usize) -> Result<&[u8], DecodeError> {
        let start_pos = self.position() as usize; // Should I check here?

        if self.get_ref().len() < start_pos + size {
            return Err(DecodeError::ShortBuffer);
        }
        self.set_position((start_pos + size) as u64);

        let buf = &self.get_ref()[start_pos..];

        Ok(&buf[..size])
    }
}

macro_rules! impl_decodable_for_tuple {
    ( $( $d: ident ),* ) => {
        impl<$($d),*> Decodable for ($($d),*)
        where $(
            $d: Decodable
        ),*
        {
            fn decode<R: ReadBuffer>(buf: &mut R) -> Result<Self, DecodeError> {
                Ok(( $( buf.read::<$d>()? ),* ))
            }
        }
    }
}

impl_decodable_for_tuple!(D1, D2);
impl_decodable_for_tuple!(D1, D2, D3);
impl_decodable_for_tuple!(D1, D2, D3, D4);
impl_decodable_for_tuple!(D1, D2, D3, D4, D5);
impl_decodable_for_tuple!(D1, D2, D3, D4, D5, D6);
impl_decodable_for_tuple!(D1, D2, D3, D4, D5, D6, D7);
impl_decodable_for_tuple!(D1, D2, D3, D4, D5, D6, D7, D8);

macro_rules! impl_decodable_for_array {
    ( $size: expr ) => {
        impl Decodable for [u8; $size] {
            fn decode<R: ReadBuffer>(buf: &mut R) -> Result<Self, DecodeError> {
                let array = unsafe{ *(buf.read_bytes($size)? as *const _ as *const [u8; $size]) };
                Ok(array)
            }
        }
    };
    ( $size: expr, $( $sizes: expr ),* ) => {
        impl_decodable_for_array!($size);
        impl_decodable_for_array!($($sizes),*);
    }
}

impl_decodable_for_array!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
impl_decodable_for_array!(32, 64, 128, 256, 512, 1024, 2048, 4096);
