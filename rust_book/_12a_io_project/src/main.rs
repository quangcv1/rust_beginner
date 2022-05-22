use std::{env, process};
use _12a_io_project::Config;
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


fn main() {
    // println!("Hello, world!");
    // let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // println!("{},{}", query, filename);

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = _12a_io_project::run(config) {
        println!("Application error: {}", e);
        process::exit(2);
    }
}
