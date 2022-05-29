use std::cmp::Ordering;
use std::convert::Infallible;

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn calling_in_loop() {
        let counter = Counter::new();
        println!("{:#?}", counter);
        let counter_iter = counter.into_iter();
        //error here
        //println!("{:#?}", counter);
        for val in counter_iter {
            println!("{}", val);
        }

    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))//4 pairs
            //(1,2); (2,3); (3,4); (4,5) and (5,None) is never produced
            //because zip returns None when either of its input iter return None
            .map(|(a,b)| a*b)
            //(2); (6); (12); (20)
            .filter(|x| x%3 == 0)
            //(6); (12)
            .sum();
        assert_eq!(18, sum);
    }
}