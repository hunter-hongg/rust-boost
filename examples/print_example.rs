use rust_boost::{prints, printlns};

fn main() {
    prints!("Hello, world!\n");
    printlns!("Hello, world!");

    let a = 5;
    let b = 5;

    prints![a, "+", b, "=", a + b, "\n"];
    printlns![a, "+", b, "=", a + b];
}