#[test]
fn struct_with_dbg() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rec1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };

    dbg!(&rec1);
    let rec2 = Rectangle {..rec1};
    dbg!(&rec2);

}