// Attributes - metadata for the compiler. When compiling 
// there might be a bunch of warning messages that'll make the
// tests you write tough to see. The #! line below cleans that up
#![allow(unused)]

// main() is the entry point of the Rust program
fn main() {
    // Macros in Rust generate code at compile time and are
    // invoked with an exclamation mark (!).
    // println! is a macro that prints text to the console
    println!("Hello, world!");
}
