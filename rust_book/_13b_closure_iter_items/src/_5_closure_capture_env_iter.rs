#[test]
fn closure_capture_env_iter() {
    #[derive(PartialEq,Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneaker".to_string(),
        },
        Shoe {
            size: 13,
            style: "sandal".to_string(),
        },
        Shoe {
            size: 10,
            style: "boot".to_string(),
        }
    ];

    let in_my_size = shoe_in_size(shoes,10);
    println!("my shoes: {:#?}", in_my_size);
}