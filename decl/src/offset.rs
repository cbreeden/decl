use std::marker::PhantomData;
use std::fmt;

use error::Error;
use declarative::DeclRead;
use declarative::DeclResult;
use declarative::Declarative;
use declarative::DeclarativeWithArgs;
use declarative::StaticEncodingSize;

macro_rules! define_offsets {
    ($($ty:ident => $final:ident, $size:expr),* $(,)*) => (
        $(
            pub struct $final<'buf, Item>
            where
                Item: DeclarativeWithArgs<'buf>,
            {
                buffer: &'buf [u8],
                argument: Item::Argument,
                phantom: PhantomData<Item>,
            }

            impl<'buf, Item> fmt::Debug for $final<'buf, Item> 
            where
                Item: DeclarativeWithArgs<'buf>,
            {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Offset")
                }
            }

            impl<'buf, Item> DeclarativeWithArgs<'buf> for $final<'buf, Item>
            where
                Item: DeclarativeWithArgs<'buf>,
            {
                type Argument = (&'buf [u8], Item::Argument);
                fn parse_with(
                    buffer: &'buf [u8],
                    argument: Self::Argument,
                ) -> DeclResult<'buf, Self> {
                    let (offset, rest) = $ty::parse(buffer)?;
                    
                    if argument.0.len() < offset as usize {
                        return Err(Error::InsufficientBytes);
                    }
                    
                    let buffer = &argument.0[offset as usize..];
                            
                    Ok((
                        $final {
                            buffer: buffer,
                            argument: argument.1,
                            phantom: PhantomData,
                        },
                        rest,
                    ))
                }
            }

            impl<'buf, Item> StaticEncodingSize for $final<'buf, Item>
            where
                Item: DeclarativeWithArgs<'buf>
            {
                const SIZE: usize = $size;
            }            
        )*
    )
}

define_offsets!(
    u8  => Offset8,  1,
    u16 => Offset16, 2,
    u32 => Offset32, 4,
    u64 => Offset64, 8,
);
