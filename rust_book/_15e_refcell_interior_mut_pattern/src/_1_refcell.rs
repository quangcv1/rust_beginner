///# RefCell<T> and the Interior Mutability Pattern
///## Interior Mutability
/// - is a design pattern that allows to mutate data even when there are
/// immutable ref to that data; normally, this action is disallowed by the borrowing rules.
/// - This type is useful when you're sure your code follows the borrowing rules
/// but the compiler is unable to understand and guarantee that.

///# RC & BOX & REFCELL
/// - Rc<T>: enables multiple owners of the same data;
/// Box<T> and RefCell<T> have single owners
/// - Box<T> allows immutable or mutable borrows checked at compile time;
/// Rc<T> allows only immutable borrows checked at compile time
/// RefCell<T> allows immutable or mutable borrows checked at runtime.
/// - Because RefCell<T> allows mutable borrows checked at runtime, you can
/// mutate the value inside the RefCell<T> even when the RefCell<T> is immutable

fn main() {}