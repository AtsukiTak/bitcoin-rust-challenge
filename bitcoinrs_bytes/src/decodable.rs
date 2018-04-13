use Bytes;

pub trait Decodable {
    fn decode(bytes: &mut Bytes) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub enum DecodeError {
    ShortLength,
    InvalidBytes,
}

pub enum Either<D1, D2> {
    Left(D1),
    Right(D2),
}

impl<D1, D2> Decodable for Either<D1, D2>
where
    D1: Decodable + Sized,
    D2: Decodable + Sized,
{
    fn decode(bytes: &mut Bytes) -> Result<Either<D1, D2>, DecodeError> {
        let mut bytes1 = *bytes;
        let mut bytes2 = *bytes;

        let err1 = match D1::decode(&mut bytes1) {
            Ok(d1) => {
                let consumed = bytes.len() - bytes1.len();
                bytes.consume(consumed);
                return Ok(Either::Left(d1));
            }
            Err(e) => e,
        };

        match D2::decode(&mut bytes2) {
            Ok(d2) => {
                let consumed = bytes.len() - bytes2.len();
                bytes.consume(consumed);
                Ok(Either::Right(d2))
            }
            Err(_) => Err(err1),
        }
    }
}

macro_rules! impl_decodable_for_tuple {
    ( $( $d: ident ),* ) => {
        impl<$($d),*> Decodable for ($($d),*)
        where $(
            $d: Decodable
        ),*
        {
            fn decode(bytes: &mut Bytes) -> Result<Self, DecodeError> {
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
