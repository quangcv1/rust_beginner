# UNRECOVERABLE WITH PANIC!
unwind and clean up the stack, and then quit  
***Unwinding***
- Rust walks back up the stack
- cleans up the data from each function it encounters
- this walking back and cleanup is a lot of work

***Aborting***
- ends the program without cleaning up
- mem that program was using will then need to be cleaned up by the OS
- if project need to make the resulting binary as small as possible, can switch from ***unwinding*** to ***aborting*** upon a panic by ***panic = 'abort'*** to the appropriate ***[profile]*** in ***Cargo.toml*** file

```rust
[profile.release]
panic = 'abort'
```

## RUST_BACKTRACE=1
RUST_BACKTRACE environment variable:
- to get a backtrace of exactly what happened to cause the error.
- is a list of all the functions that have been called to get to this point
- key to reading the backtrace is to start from the top and read until you see files you wrote
- the lines above that spot are code that your code has called
- the lines below are code that called your code

In order to get backtraces, debug symbols must be enabled  
Debug symbols are enabled by default when using ***cargo build** or ***cargo run*** without the ***--release*** flag.