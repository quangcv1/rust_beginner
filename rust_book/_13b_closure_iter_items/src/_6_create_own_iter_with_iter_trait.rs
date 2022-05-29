use std::cmp::Ordering;
use std::convert::Infallible;


///# Creating Own Iterators: With Iterator Trait
///- by calling iter, into_iter, iter_mut on a vector
#[test]
fn create_own_iter() {
    #[derive(Debug)]
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
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

    let counter = Counter::new();
    println!("{:#?}", counter);
    let counter_iter = counter.into_iter();
    //error here
    //println!("{:#?}", counter);
    for val in counter_iter {
        println!("{}", val);
    }

    //another way
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}