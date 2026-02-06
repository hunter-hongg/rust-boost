#[macro_export]
macro_rules! prints {
    ($($arg:expr),*) => {
        ::std::print!("\r");
        $(
            ::std::print!("{} ", $arg);
        )*
    };
}

#[macro_export]
macro_rules! printlns {
    ($($arg:expr),*) => {
        ::std::print!("\r");
        $(
            ::std::print!("{} ", $arg);
        )*
        ::std::println!("");
    };
}

pub fn print(value: impl std::fmt::Display)  {
    print!("{}", value);
}

pub fn println(value: impl std::fmt::Display)  {
    println!("{}", value);
}