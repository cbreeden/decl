#![allow(warnings)]

extern crate byteorder;

mod error;
pub mod declarative;
pub mod array;
pub mod primitives;
pub mod ignored;
pub mod offset;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
