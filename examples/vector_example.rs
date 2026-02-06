use rust_boost::{container::vector::Vector, printlns, vector};

fn main() {
    let mut v: Vector<i32> = vector![-1, 0, 1, 2, 3];

    v.print();

    v.map_self(|x| x * 10);

    v.print();

    v.fliter_self(|x| x.clone() > 2);

    v.print();

    printlns!(
        "len:",
        v.len(),
        "[0]:",
        v.at(0)
            .and_then(|x| Some(x.to_string()))
            .unwrap_or("越界".to_string()),
        "[99]:",
        v.at(99)
            .and_then(|x| Some(x.to_string()))
            .unwrap_or("越界".to_string())
    );
}
