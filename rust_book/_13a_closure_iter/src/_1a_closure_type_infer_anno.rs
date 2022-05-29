///# CLOSURE Type Inference and Annotation
/// - Closures don't require to annotate type
/// - Don't require the return value
/// - Types are locked into the closure in the first execution
/// And we get a type error if we try to use a different type with the same closure

#[test]
fn closure_type_infer_anno() {
    fn add_one_v1 (x: u32) -> u32 {x+1} //fn
    let add_one_v2 = |x: u32| -> u32 {x+1}; //closure v1
    //let add_one_v3 = |x| {x + 1}; //closure v2
    //let add_one_v4 = |x| x+1; //closure v3

}

#[test]
fn closure_type_infer_anno_error() {
    let example_closure = |x| x;
    let s = example_closure("hello".to_string());
    //let n = example_closure(5);
}