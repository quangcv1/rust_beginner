///# PACKAGES AND CRATES
///```rust
///cargo new my-project
///```
///- Cargo.toml: giving us a package
///- src/main.rs is the ***crate root*** of a binary crate with the same name as the package
///- src/lib.rs (is the package dir contains) a library crate with the same name as the package and is it's crate root
///- Cargo passes the crate root files to ***rustc*** to build the lib or binary
///- If the package contains ***src/main.rs*** and ***src/lib.rs***, it has 2 crates:
/// a binary and a library both with the same name as the package
