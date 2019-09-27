use reify::ast;
use reify::Reify;

#[derive(Reify)]
struct Sub1(u8, String);

#[derive(Reify)]
enum Sub2 {
    A(u8),
    B { x: u8 },
    C,
}

#[derive(Reify)]
struct Demo {
    a: u8,
    b: String,
    c: Sub1,
    d: Sub2,
    e: (u8, String),
    f: Vec<u8>,
}

fn main() {
    println!("ast = {:?}", ast::<Demo>());
}
