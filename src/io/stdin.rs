use crate::{
    container::vector::Vector,
    error::{errors::Error, result::Result},
};
use std::io;

pub fn scanln() -> Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(error) => Err(Error::ErrorInp(error.to_string())),
    }
}

pub fn scanlns() -> Result<Vector<String>> {
    let inp = scanln()?;
    Ok(Vector::new(
        inp.split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    ))
}
