#[test]
fn in_method() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl <T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point {x:5, y:10};
    println!("p.x = {}", p.x());

    let pf: Point<f64> = Point {x:1.2, y:2.3};
    println!("distance from origin = {}", pf.distance_from_origin())
}