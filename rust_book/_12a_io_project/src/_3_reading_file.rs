use std::fs;

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn reading_file() {
        let content = fs::read_to_string("a.txt")
            .expect("something went wrong reading the file!");
        println!("{:#?}", content);
    }

    // #[test]
    // fn reading_file_advance() {
    //     let content = fs::read_to_string("a.txt")
    //         ?.
    // }
}

