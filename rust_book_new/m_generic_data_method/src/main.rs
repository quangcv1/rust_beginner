/**
# IN METHOD DEFINITION
- struct/ enum -> implement method WITH generic data types
*/

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



fn main() {
    let p = Point{x:5, y:10};
    println!("{}", p.x);
    println!("p.x = {}", p.x());

    let pt = Point{x:1.2, y:4.3};
    println!("distance from origin: {} and {}", pt.distance_from_origin(), p.distance_from_origin());

    let s = Point{x: String::from("x"), y: String::from("y"),};
    let t = &s.x;
    println!("{}", s.x);
    println!("{} !", s.x);
    println!("s.x = {}", s.x());

}


// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn main() {
//     let p = Point { x: 5, y: 10 };
//
//     println!("p.x = {}", p.x());
// }
