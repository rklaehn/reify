use std::mem;
use std::marker::PhantomData;

pub use reify_derive::*;

// pub trait Reify {
//     fn ast(_: PhantomData<&Self>) -> &'static str;
// }

// impl Reify for u32 {
//     fn ast(_: PhantomData<&Self>) -> &'static str {
//         "u32"
//     }
// }

// impl Reify for i32 {
//     fn ast(_: PhantomData<&Self>) -> &'static str {
//         "i32"
//     }
// }

// fn ast<T: Reify>() -> &'static str {
//     let x: PhantomData<&T> = PhantomData;
//     Reify::ast(x)
// }

pub trait Reify {
    /// Total number of bytes of heap memory owned by `self`.
    ///
    /// Does not include the size of `self` itself, which may or may not be on
    /// the heap. Includes only children of `self`, meaning things pointed to by
    /// `self`.
    fn ast(_: PhantomData<&Self>) -> usize;
}

pub fn ast<T: Reify>() -> usize {
    let x: PhantomData<&T> = PhantomData;
    Reify::ast(x)
}

//
// In a real version of this library there would be lots more impls here, but
// here are some interesting ones.
//

impl Reify for u8 {
    /// A `u8` does not own any heap memory.
    fn ast(_: PhantomData<&Self>) -> usize {
        0
    }
}

impl Reify for String {
    /// A `String` owns enough heap memory to hold its reserved capacity.
    fn ast(_: PhantomData<&Self>) -> usize {
        1
    }
}

impl<T> Reify for Box<T>
where
    T: ?Sized + Reify,
{
    /// A `Box` owns however much heap memory was allocated to hold the value of
    /// type `T` that we placed on the heap, plus transitively however much `T`
    /// itself owns.
    fn ast(_: PhantomData<&Self>) -> usize {
        2
    }
}

impl<T> Reify for [T]
where
    T: Reify,
{
    /// Sum of heap memory owned by each element of a dynamically sized slice of
    /// `T`.
    fn ast(_: PhantomData<&Self>) -> usize {
        3
    }
}

impl<'a, T> Reify for &'a T
where
    T: ?Sized,
{
    /// A shared reference does not own heap memory.
    fn ast(_: PhantomData<&Self>) -> usize {
        4
    }
}
