///# LIFETIME RULES
/// - ***input lifetimes***: lifetimes on fn or method para
/// - ***output lifetimes***: lifetimes on return values
/// ## 1 Rule -- apply to input lifetimes
/// - Complier assigns a lifetime parameter to each para that's a reference.
/// - a fn with one para gets one lifetime para ***fn foo<'a>(x: &'a i32);***
/// - a fn with 2 para get 2 separate lifetime para ***fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
///
/// ## 2 rule -- apply to output lifetime
/// - if there is exactly one input para, that lifetime is assigned to all output lifetime para
/// - ex: ***fn foo<'a>(x: &'a i32) -> &'a i32.
///## 3 rule -- apply to output lifetime
/// - if there are multiple input lifetime para, but one of them is ***&self*** or
/// ***&mut self*** because this is a method
/// => the lifetime of ***self*** is assigned to all output lifetime para
///
#[test]
fn test() {}