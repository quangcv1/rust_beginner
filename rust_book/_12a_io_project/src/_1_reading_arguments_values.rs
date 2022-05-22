///# TESTING USING CML
/// -cargo test saving_arguments -- --show-output
use std::env;

#[test]
fn reading_arguments_values() {
    let args= env::args();
    println!("{:#?}", args);
    let args_collect: Vec<String> = args.collect();
    println!("{:#?}", args_collect);
}