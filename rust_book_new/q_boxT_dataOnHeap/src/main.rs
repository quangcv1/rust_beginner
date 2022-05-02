fn main() {
    /// Box on the Heap
    /// # Description:
    /// b have the value of a Box (pointer address maybe) that points to the value 5
    /// which is allocated on the heap
    /// - Box (stored on the stack - pointer address)
    /// - Box is a pointer -> so Rust always knows how much space a Box needs
    /// a pointer's size doesn't change based on the amount of data it's pointing to
    /// - data it points to (stored on the heap)
    let b = Box::new(5);
    println!("{}", b);

    /// # USE CASE
    /// ## Description:
    /// - At compile time, Rust needs to know how much space a type takes up
    /// - One type whose size can't be known at compile time is a
    /// ***recursive type***
    ///
}
