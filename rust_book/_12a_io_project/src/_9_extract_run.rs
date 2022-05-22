use std::error::Error;
use std::{env, fs, process};

#[test]
fn extract_run() {
    struct Config_n {
        query: String,
        filename: String,
    }

    impl Config_n {
        fn new_n(args: &[String]) -> Result<Config_n, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            Ok(
                Config_n {
                    query: args[1].clone(),
                    filename: args[2].clone(),
                }
            )
        }
    }

    //Box<dyn Error> means the fn will return a type that implements
    //the Error trait
    //but we don't have to specify what particular type the return value will be
    fn run(config: Config_n) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        println!("With text:\n{}", contents);
        Ok(())
    }

    let args: Vec<String> = env::args().collect();
    let config_n = Config_n::new_n(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    println!("Searching for {}", config_n.query);
    println!("In file {}", config_n.filename);

    if let Err(e) = run(config_n) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}