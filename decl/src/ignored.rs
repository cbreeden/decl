use std::fmt;
use std::marker::PhantomData;

use error::Error;
use declarative::DeclResult;
use declarative::StaticEncodingSize;
use declarative::Declarative;

pub struct Ignored<T>(PhantomData<T>);

impl<T> fmt::Debug for Ignored<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ignored")
    }
}

// TODO: Is this trait implementation really necessary? This will primarily
//       be handled on the procedural macro end.
impl<'buf, T> Declarative<'buf> for Ignored<T>
where
    T: Declarative<'buf>,
    T: StaticEncodingSize,
{
    fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
        if buffer.len() < Self::SIZE {
            return Err(Error::InsufficientBytes);
        }

        Ok((Ignored(PhantomData), &buffer[Self::SIZE..]))
    }
}

impl<T> StaticEncodingSize for Ignored<T>
where
    T: StaticEncodingSize
{
    const SIZE: usize = T::SIZE;
}
