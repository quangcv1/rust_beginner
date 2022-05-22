use std::env;

#[test]
fn test_parse_config() {
    fn parse_config_test(args: &[String]) -> (&str, &str){
        let query = &args[1];
        let filename = &args[2];
        (query,filename)
    }



    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    let (query, filename) = parse_config_test(&args);
    println!("{}, {}", query, filename);
}
