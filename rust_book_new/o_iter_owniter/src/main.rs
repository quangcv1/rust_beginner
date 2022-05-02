use std::cmp::Ordering;
use std::convert::Infallible;
use std::iter::{Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect, Map, MapWhile, Peekable, Product, Rev, Scan, Skip, SkipWhile, StepBy, Sum, Take, TakeWhile, Zip};

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter{count:0}
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    println!("Hello, world!");
    let sum1: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .collect();
    println!("{:?}", sum1);
    let sum1: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a * b)
        .collect();
    println!("{:?}", sum1);
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a * b)
        .filter(|x| x%3 == 0)
        .sum();
    //assert_eq!(18,sum);
    println!("{:?}", sum);
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// #[test]
// fn using_other_iterator_trait_methods() {
//     let sum: u32 = Counter::new()
//         .zip(Counter::new().skip(1))
//         .map(|(a,b)| a * b)
//         .filter(|x| x%3 == 0)
//         .sum();
//     assert_eq!(18,sum);
//
// }