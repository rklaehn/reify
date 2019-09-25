use reify::ast;
use reify::Reify;

#[derive(Reify)]
struct Sub1(u8, String);

#[derive(Reify)]
enum Sub2 {
    A,
    B,
}

#[derive(Reify)]
struct Demo {
    a: u8,
    b: String,
    c: Sub1,
    d: Sub2,
}

fn main() {
    // 10 + 0 + 0 + 6 = 16
    println!("ast = {:?}", ast::<Demo>());
}
