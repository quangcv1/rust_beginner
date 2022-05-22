use std::env;

#[test]
fn test_struct_config() {
    #[derive(Debug)]
    struct Config<'a> {
        query: &'a String,
        filename: &'a String,
    }

    fn parse_config(args: &[String]) -> Config {
        let query = &args[1];
        let filename = &args[2];
        Config {
            query,
            filename,
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("{:#?}",config);

}

#[test]
fn struct_config_copy() {
    #[derive(Debug)]
    struct Config {
        query:  String,
        filename:  String,
    }

    fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {
            query,
            filename,
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("{:#?}",config);
}