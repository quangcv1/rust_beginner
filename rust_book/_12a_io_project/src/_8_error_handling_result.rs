use std::{env, process};

#[test]
fn err_handling_result() {
    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            Ok(
                Config {
                    query: args[1].clone(),
                    filename: args[2].clone(),
                }
            )
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);//stop the program and return the number 1 as the exit status code
        });
    //unwrap_or_else:
    //Ok -> this method's behavior is similar to unwrap: return inner value Ok is wrapping
    //Err -> calls the code in the closure, which is an anonymous fn

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename)
}