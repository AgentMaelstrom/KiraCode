/// # main.rs
///
/// This is the main file for the KiraCode compiler, version 0.

// ---

// __ Imports __
use std::env;

/// ## main -> () Function
///
/// The main function of the compiler program.
fn main() {
    // __ Constants __
    static VERSION: &str = "test0";

    // &args[0] is the path of the file or the name of the command
    let args: Vec<String> = env::args().collect();

    dbg!(args);
}
