use {ReadBuf, ReadError};

pub trait Decodable {
    fn decode<R>(bytes: &mut R) -> Result<Self, ReadError>
    where
        Self: Sized,
        R: ReadBuf;
}

macro_rules! impl_decodable_for_tuple {
    ( $( $d: ident ),* ) => {
        impl<$($d),*> Decodable for ($($d),*)
        where $(
            $d: Decodable
        ),*
        {
            fn decode<R: ReadBuf>(bytes: &mut R) -> Result<Self, ReadError> {
                Ok(( $( bytes.read::<$d>()? ),* ))
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
