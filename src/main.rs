#![allow(unused)]

use std::env;

fn div(x: u32, y: u32) -> u32 {
    x / y
}

// Function that doesn't return any output
fn print(s: String) {
    println!("{0}{0}{0}{0}{0}", s);
}

fn main() {
    let x: u32 = 5;
    let y: u32 = 5;
    let z: u32 = div(x, y);

    println!("{} / {} = {}", x, y, z);

    print("🐸".to_string());

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Hello, {}!", args[1]);
    } else {
        println!("Hello, Rust!");
    }
}