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
    Array(Vec<Ast2>),
    Scalar(&'static str),
    Unit
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
