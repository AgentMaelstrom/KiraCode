/// # main.rs File
///
/// This is the main file for the KiraCode compiler, version 'test0'.
// ---

// Imports
use std::env;

// Internal Module Imports
mod generate;
mod parse;
mod tokenise;

use generate::*;
use parse::*;
use tokenise::*;

// ---

/// ## main -> () Function
///
/// The main function of the compiler program.
fn main() -> () {
    // __ Constants __
    static VERSION: &str = "test0";

    // &args[0] is the path of the file or the name of the command
    let args: Vec<String> = env::args().collect();

    dbg!(args);
}
