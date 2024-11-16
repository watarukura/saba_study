#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

use noli::prelude::*;

fn main() {
    SystemApi::write_string("Hello, world\n");
    println!("Hello from println!");
    SystemApi::exit(42);
}

noli::entry_point!(main);
