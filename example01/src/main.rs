use std::env;

pub fn main() {
    let _args: Vec<String> = env::args().collect();

    // The Rust equivalent of the 'return 0;' in C main:
    std::process::exit(0);
}
