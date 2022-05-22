use std::{env, fs};

#[test]
fn error_handling() {
    #[derive(Debug)]
    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            if args.len() < 3 {
                panic!("not enough arguments!");
            }
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    println!("args:\n{:#?}", args);
    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading file");
    println!("With text:\n{}", contents);
}