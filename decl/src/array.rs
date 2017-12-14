use std::marker::PhantomData;
use std::fmt;

use error::Error;
use declarative::DeclRead;
use declarative::DeclResult;
use declarative::DeclarativeWithArgs;
use declarative::StaticEncodingSize;
use declarative::DynamicEncodingSize;


// TODO:
//  [ ] Implement linear search?
//  [ ] Implement binary search?
//  [ ] Implement .get(usize) interface?

pub struct Array<'buf, Item>
where
    Item: DeclarativeWithArgs<'buf>,
{
    buffer: &'buf [u8],
    length: usize,
    argument: Item::Argument,
    phantom: PhantomData<Item>,
}

impl<'buf, Item> fmt::Debug for Array<'buf, Item> 
where
    Item: DeclarativeWithArgs<'buf>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array {{ length: {} }}", self.length)
    }
}

impl<'buf, Item> DeclarativeWithArgs<'buf> for Array<'buf, Item>
where
    Item: DeclarativeWithArgs<'buf>,
{
    type Argument = (usize, Item::Argument);
    fn parse_with(
        buffer: &'buf [u8],
        arguments: (usize, Item::Argument),
    ) -> DeclResult<'buf, Self> {
        Ok((
            Array {
                buffer: buffer,
                length: arguments.0,
                argument: arguments.1,
                phantom: PhantomData,
            },
            buffer,
        ))
    }
}

impl<'buf, Item> DynamicEncodingSize for Array<'buf, Item>
where
    Item: StaticEncodingSize,
    Item: DeclarativeWithArgs<'buf>,
{
    fn size(&self) -> usize {
        Item::SIZE * self.length
    }
}

impl<'buf, Item> IntoIterator for Array<'buf, Item>
where
    Item: StaticEncodingSize,
    Item: DeclarativeWithArgs<'buf>,
    Item::Argument: Clone,
{
    type IntoIter = ArrayIter<'buf, Item>;
    type Item = Result<Item, Error>;
    fn into_iter(self) -> Self::IntoIter {
        ArrayIter {
            buffer: self.buffer,
            length: self.length,
            argument: self.argument,
            cursor: 0usize,
            phantom: PhantomData,
        }
    }
}

pub struct ArrayIter<'buf, Item>
where
    Item: DeclarativeWithArgs<'buf>,
{
    buffer: &'buf [u8],
    length: usize,
    argument: Item::Argument,
    cursor: usize,
    phantom: PhantomData<Item>,
}

impl<'buf, Item> Iterator for ArrayIter<'buf, Item>
where
    Item: StaticEncodingSize,
    Item: DeclarativeWithArgs<'buf>,
    Item::Argument: Clone,
{
    type Item = Result<Item, Error>;
    fn next(&mut self) -> Option<Result<Item, Error>> {
        if self.length <= self.cursor {
            return None;
        }

        self.cursor += 1;
        let dest = self.buffer.parse_with::<Item>(self.argument.clone());
        Some(dest)
    }
}

/// An unbounded variant of an `Array`.  This type acts like a `&[T]` except that it
/// doesn't implement an iterator interface as the size of the array isn't known.
/// Instead, we implement only getters.
pub struct Slice<'buf, Item>
where
    Item: DeclarativeWithArgs<'buf>,
{
    buffer: &'buf [u8],
    argument: Item::Argument,
    phantom: PhantomData<Item>,
}

impl<'buf, Item> fmt::Debug for Slice<'buf, Item> 
where
    Item: DeclarativeWithArgs<'buf>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Slice")
    }
}


impl<'buf, Item> DeclarativeWithArgs<'buf> for Slice<'buf, Item>
where
    Item: DeclarativeWithArgs<'buf>,
{
    type Argument = Item::Argument;
    fn parse_with(
        buffer: &'buf [u8],
        argument: Item::Argument,
    ) -> DeclResult<'buf, Self> {
        Ok((
            Slice {
                buffer: buffer,
                argument: argument,
                phantom: PhantomData,
            },
            buffer,
        ))
    }
}
