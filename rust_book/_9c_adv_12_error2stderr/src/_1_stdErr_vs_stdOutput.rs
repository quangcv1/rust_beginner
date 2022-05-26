///# STANDARD ERROR VS STANDARD OUTPUT
/// https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html
///

use std::{env, process};

#[test]
fn stdErr_vs_stdOutput() {
    eprintln!("Problem passing argument");
    process::exit(1);
}