use std::env;

fn main() {
    // Turn on debug mode, if app rus with '-d' argument
    let debug = env::args().any(|arg| arg == "--debug");

    // Print information
    if !debug {println!("Booting v{};",                env!("CARGO_PKG_VERSION"));}
    else      {println!("Booting v{}, in debug mode;", env!("CARGO_PKG_VERSION"));}
}
