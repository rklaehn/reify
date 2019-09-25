use reify::ast;
use reify::Reify;

#[derive(Reify)]
struct Demo {
    a: u8,
    b: String,
}

fn main() {
    // 10 + 0 + 0 + 6 = 16
    println!("heap size = {}", ast::<Demo>());
}
