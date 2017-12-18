use error::Error;
use array::Array;

use std::io::Read;
use std::fmt::Debug;

pub type DeclResult<'buf, T> = Result<(T, &'buf [u8]), Error>;

/// Implemented on types whose encoding size is known _prior_
/// to being parsed.
pub trait StaticEncodingSize {
    const SIZE: usize;
}

/// Implemented on types whose encoding size is known after
/// it is provided an argument.  For example Array<T> doesn't
/// know it's encoding size until the length of the array is known.
pub trait DynamicEncodingSize {
    fn size(&self) -> usize;
}

impl<T> DynamicEncodingSize for T
where
    T: StaticEncodingSize,
{
    fn size(&self) -> usize {
        Self::SIZE
    }
}

pub trait Declarative<'buf>: Sized {
    fn parse(&'buf [u8]) -> DeclResult<'buf, Self>;
}

pub trait DeclarativeWithArgs<'buf>: Sized {
    type Argument;
    fn parse_with(&'buf [u8], Self::Argument) -> DeclResult<'buf, Self>;
}

impl<'buf, T> DeclarativeWithArgs<'buf> for T
where
    T: Declarative<'buf>,
{
    type Argument = ();
    fn parse_with(buffer: &'buf [u8], argument: Self::Argument) -> DeclResult<'buf, Self> {
        Self::parse(buffer)
    }
}

pub trait DeclRead<'buf>: Sized {
    fn parse<T>(&mut self) -> Result<T, Error>
    where
        T: Declarative<'buf>;

    fn parse_with<T>(&mut self, T::Argument) -> Result<T, Error>
    where
        T: DeclarativeWithArgs<'buf>;

    fn parse_array<T>(&mut self, length: usize) -> Result<Array<'buf, T>, Error>
    where
        T: Declarative<'buf> + StaticEncodingSize,
    {
        DeclRead::parse_with::<Array<T>>(self, (length, ()))
    }

    fn parse_array_with<T>(
        &mut self,
        length: usize,
        argument: T::Argument,
    ) -> Result<Array<'buf, T>, Error>
    where
        T: DeclarativeWithArgs<'buf> + StaticEncodingSize,
    {
        DeclRead::parse_with::<Array<T>>(self, (length, argument))
    }

    fn tag<T>(&mut self, tag: T) -> Result<&[u8], Error>
    where
        T: Tag;
}

impl<'buf> DeclRead<'buf> for &'buf [u8] {
    fn parse<T>(&mut self) -> Result<T, Error>
    where
        T: Declarative<'buf>,
    {
        let (result, rest) = T::parse(self)?;
        *self = rest;
        Ok(result)
    }

    fn parse_with<T>(&mut self, argument: T::Argument) -> Result<T, Error>
    where
        T: DeclarativeWithArgs<'buf>,
    {
        let (result, rest) = T::parse_with(self, argument)?;
        *self = rest;
        Ok(result)
    }

    fn tag<T>(&mut self, tag: T) -> Result<&[u8], Error>
    where
        T: Tag,
    {
        Tag::validate(&tag, self)
    }
}

pub trait Tag {
    fn validate<'b>(&self, buffer: &'b [u8]) -> Result<&'b [u8], Error>;
}

impl Tag for str {
    fn validate<'b>(&self, buffer: &'b [u8]) -> Result<&'b [u8], Error> {
        if buffer.len() < self.len() {
            return Err(Error::InsufficientBytes)
        }

        match &buffer[..self.len()] == self.as_bytes() {
            true => Ok(&buffer[self.len()..]),
            false => Err(Error::InvalidVersion),
        }
    }
}

impl Tag for [u8] {
    fn validate<'b>(&self, buffer: &'b [u8]) -> Result<&'b [u8], Error> {
        if buffer.len() < self.len() {
            return Err(Error::InsufficientBytes)
        }

        match &buffer[..self.len()] == self {
            true => Ok(&buffer[self.len()..]),
            false => Err(Error::InvalidVersion),
        }
    }
}


macro_rules! impl_tag_primitive {
    ($($ty:ty),*) => {
        $(
            impl Tag for $ty {
                fn validate<'b>(&self, buffer: &'b [u8]) -> Result<&'b [u8], Error> {
                    Self::parse(buffer).map(|(_, buffer)| buffer)
                }
            }
        )*
    };
}

impl_tag_primitive!(u8, u16, u32, u64, i8, i16, i32, i64);