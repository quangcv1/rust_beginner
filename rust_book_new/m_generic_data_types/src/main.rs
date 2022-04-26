/**
# GENERIC DATA TYPES:
## Biz case:
- one function that can be used to find the largest number/ char in a list
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largestRef<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    let resultRef = largestRef(&number_list);
    println!("The largest number is {}", result);
    println!("The largest number is {}", resultRef);

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
}
