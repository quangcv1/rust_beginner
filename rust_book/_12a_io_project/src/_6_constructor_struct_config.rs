use std::env;

#[test]
fn constructor_struct_config() {
    #[derive(Debug)]
    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("{:#?}",config);
}