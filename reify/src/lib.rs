
use std::marker::PhantomData;

pub use reify_derive::*;

#[derive(Debug, Clone)]
pub enum Fields {
    Unit,
    Unnamed(Vec<Ast2>),
    Named(Vec<(&'static str, Ast2)>),
}

#[derive(Debug, Clone)]
pub enum Ast2 {
    Struct(Fields),
    Enum(Vec<(&'static str, Fields)>),
    Tuple(Vec<Ast2>),
    Array(Box<Ast2>),
    Scalar(&'static str),
    Unit
}

impl<A: Reify, B:Reify> Reify for (A, B) {
    fn ast(_: PhantomData<&Self>) -> Ast2 {
        Ast2::Tuple(vec![ast::<A>(), ast::<B>()])
    }
}

impl<A: Reify> Reify for Vec<A> {
    fn ast(_: PhantomData<&Self>) -> Ast2 {
        Ast2::Array(Box::new(ast::<A>()))
    }
}

pub trait Reify {
    fn ast(_: PhantomData<&Self>) -> Ast2;
}

pub fn ast<T: Reify>() -> Ast2 {
    let x: PhantomData<&T> = PhantomData;
    Reify::ast(x)
}

impl Reify for u8 {
    fn ast(_: PhantomData<&Self>) -> Ast2 {
        Ast2::Scalar("u8")
    }
}

impl Reify for String {
    fn ast(_: PhantomData<&Self>) -> Ast2 {
        Ast2::Scalar("String")
    }
}
