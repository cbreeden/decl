use declarative::Declarative;
use declarative::DeclResult;
use declarative::DeclRead;
use declarative::StaticEncodingSize;
use error::Error;
use byteorder::{LE, BE, NativeEndian, ByteOrder};

fn read_u8(buf: &[u8]) -> u8 {
    buf[0]
}

fn read_i8(buf: &[u8]) -> i8 {
    buf[0] as i8
}

macro_rules! declare_primitives {
    ($($func:path => $final:ident, $size:expr),* $(,)*) => {
        $(
            impl<'buf> Declarative<'buf> for $final {
                fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
                    if buffer.len() < Self::SIZE {
                        return Err(Error::InsufficientBytes);
                    }

                    let dest = $func(buffer);
                    Ok((dest, &buffer[Self::SIZE..]))
                }
            }

            impl StaticEncodingSize for $final {
                const SIZE: usize = $size;
            }
        )*
    };
}

// TODO: Remove the size field once static functions are stabalized.
// TODO: By default, integer parsing is in Big Endian.  Use compiler flags
//       to change the default to Little Endian or Native Endian.

declare_primitives!(
    read_u8      => u8,  1,
    read_i8      => i8,  1,
    BE::read_u16 => u16, 2,
    BE::read_i16 => i16, 2,
    BE::read_u32 => u32, 4,
    BE::read_i32 => i32, 4,
    BE::read_u64 => u64, 8,
    BE::read_i64 => i64, 8,
);

macro_rules! define_wrappers {
    ($($func:path => $final:ident, $ty:ty, $size:expr),* $(,)*) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $final($ty);
            
            impl From<$ty> for $final {
                fn from(ty: $ty) -> $final {
                    $final(ty)
                }
            }
            
            impl<'buf> Declarative<'buf> for $final {
                fn parse(buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
                    if buffer.len() < Self::SIZE {
                        return Err(Error::InsufficientBytes);
                    }

                    let dest = $func(buffer);
                    Ok(($final(dest), &buffer[Self::SIZE..]))
                }
            }

            impl StaticEncodingSize for $final {
                const SIZE: usize = $size;
            }
        )*
    };
}

define_wrappers!(
    BE::read_u16 => BeU16, u16, 2,
    BE::read_i16 => BeI16, i16, 2,
    BE::read_u32 => BeU32, u32, 4,
    BE::read_i32 => BeI32, i32, 4,
    BE::read_u64 => BeU64, u64, 8,
    BE::read_i64 => BeI64, i64, 8,

    LE::read_u16 => LeU16, u16, 2,
    LE::read_i16 => LeI16, i16, 2,
    LE::read_u32 => LeU32, u32, 4,
    LE::read_i32 => LeI32, i32, 4,
    LE::read_u64 => LeU64, u64, 8,
    LE::read_i64 => LeI64, i64, 8,

    NativeEndian::read_u16 => NeU16, u16, 2,
    NativeEndian::read_i16 => NeI16, i16, 2,
    NativeEndian::read_u32 => NeU32, u32, 4,
    NativeEndian::read_i32 => NeI32, i32, 4,
    NativeEndian::read_u64 => NeU64, u64, 8,
    NativeEndian::read_i64 => NeI64, i64, 8,
);

macro_rules! impl_tuple {
    ($($tup:ident),*) => (
            impl<'buf, $($tup),*> Declarative<'buf> for ($($tup),*)
            where
                $(
                $tup: Declarative<'buf>,
                )*
            {
                fn parse(mut buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
                    
                    Ok((
                        ($(
                            DeclRead::parse::<$tup>(&mut buffer)?
                        ),*),
                        buffer,
                    ))
                }
            }
    )
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
