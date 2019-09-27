use std::marker::PhantomData;

pub use reify_derive::*;

#[derive(Debug, Clone)]
pub enum Fields {
    Unit,
    Unnamed(Vec<Ast>),
    Named(Vec<(&'static str, Ast)>),
}

#[derive(Debug, Clone)]
pub enum Ast {
    Struct(Fields),
    Enum(Vec<(&'static str, Fields)>),
    Tuple(Vec<Ast>),
    Array(Box<Ast>),
    Scalar(&'static str),
    Unit,
}

macro_rules! tuple_impls {
    ( $head:ident, $( $tail:ident, )* ) => {
        impl<$head: Reify, $( $tail: Reify ),*> Reify for ($head, $( $tail ),*)
        {
            fn ast(_: PhantomData<&Self>) -> Ast {
                Ast::Tuple(vec![ast::<$head>(), $( ast::<$tail>(), )*])
            }
        }

        tuple_impls!($( $tail, )*);
    };

    () => {};
}

tuple_impls!(A, B, C, D, E, F, G, H, I, J,);

impl<A: Reify> Reify for Vec<A> {
    fn ast(_: PhantomData<&Self>) -> Ast {
        Ast::Array(Box::new(ast::<A>()))
    }
}

pub trait Reify {
    fn ast(_: PhantomData<&Self>) -> Ast;
}

pub fn ast<T: Reify>() -> Ast {
    let x: PhantomData<&T> = PhantomData;
    Reify::ast(x)
}

impl Reify for u8 {
    fn ast(_: PhantomData<&Self>) -> Ast {
        Ast::Scalar("u8")
    }
}

impl Reify for String {
    fn ast(_: PhantomData<&Self>) -> Ast {
        Ast::Scalar("String")
    }
}
