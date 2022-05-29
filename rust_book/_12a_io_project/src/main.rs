use std::{env, process};
use crate::io_mod::io_mod::parse_config;

mod _1_reading_arguments_values;
mod _2_saving_arguments_in_var;
mod _3_reading_file;
mod io_mod;
mod _4_parse_config;
mod _5_struct_config;
mod _6_constructor_struct_config;
mod _7_error_handling_panic;
mod _8_error_handling_result;
mod _9_extract_run;
mod solutions;


fn main() {
    // println!("Hello, world!");
    // let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // println!("{},{}", query, filename);


    let args: Vec<String> = env::args().collect();
    ///# BASIC APPROACH
    let config_basic = solutions::basic::Config::new(&args)
        .unwrap_or_else(|err| {
            //std_error vs std output https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html
            eprintln!("Basic: Problem parsing arguments: {}", err);
            process::exit(1);
        });

    println!("Basic: Searching for {}", config_basic.query);
    println!("Basic: In file {}", config_basic.filename);

    if let Err(e) = solutions::basic::run(config_basic) {
        eprintln!("Basic: Application error: {}", e);
        process::exit(2);
    }

    ///# ADV APPROACH
    let config_adv = solutions::adv_1::Config::new(env::args())
        .unwrap_or_else(|err| {
            //std_error vs std output https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html
            eprintln!("Adv1: Problem parsing arguments: {}", err);
            process::exit(1);
        });

    println!("Adv1: Searching for {}", config_adv.query);
    println!("Adv1: In file {}", config_adv.filename);

    if let Err(e) = solutions::adv_1::run(config_adv) {
        eprintln!("Adv1: Application error: {}", e);
        process::exit(2);
    }

}
