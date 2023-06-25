#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::env;
mod engine;

fn main() {
    // Turn on debug mode, if app rus with '-d' argument
    let debug = env::args().any(|arg| arg == "--debug");

    // Print information
    if !debug {println!("Booting v{};",                env!("CARGO_PKG_VERSION"));}
    else      {println!("Booting v{}, in debug mode;", env!("CARGO_PKG_VERSION"));}
    
    // Run engine
    engine::Init(debug);
}

