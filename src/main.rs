use std::env;
use std::fs;

mod parser;
mod token;
// mod token;
fn main() {
    let args = env::args().nth(1);

    let file = if let Some(f) = args {
        f
    } else {
        panic!("Provide proper args!");
    };

    let maybe_content = fs::read_to_string(file);
    let content = if maybe_content.is_ok() {
        maybe_content.unwrap()
    } else {
        panic!("File not found!");
    };
}