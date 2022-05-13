use std::fmt::{Debug, Display};

#[test]
fn trait_with_where_clause() {
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {32}

    ///# TRAIT WITH WHERE CLAUSE
    /// Check it out
    fn some_function_where<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
                U: Clone + Debug
    {
        32
    }
}