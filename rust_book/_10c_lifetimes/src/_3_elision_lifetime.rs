///# LIFETIME ELISION RULES
/// - These aren't rules for programmers to follow
/// - They're set of particular cases that compiler will consider, and if your code fits these cases, you don't need to write the lifetimes explicitly
///# fn first_word<'a>(s: &'a str) -> &'a str {
/// vs
/// ## fn first_word(s: &str) -> &str {
/// these situations were predictable and followed a few deterministic patterns
/// The developers programmed these patterns into the compiler's code
/// so the borrow checker could infer the lifetimes in these situations and
/// wouldn't need explicit annotations

#[test]
fn lifetime_elision() {
    //fn first_word<'a>(s: &'a str) -> &'a str {
    fn first_word_elision(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}