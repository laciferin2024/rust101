/*
Run this file with RUST_BACKTRACE=full
*/
use color_eyre::install;

fn main() {
    install();
    None::<i32>.unwrap(); //panic
}
