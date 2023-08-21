/// # main.rs
///
/// This is the main file for the KiraCode compiler, version 0.
// ----------------

// __ Outline __
//
// Imports ... 013
// Constants . 020

// ----------------

// __ Imports __
use std::env;

// ----------------

// __ Enums __

/// ## Status Enum
///
/// - `Good`              - Status is good
/// - `Bad(&'static str)` - Status is bad, say why (`&'static str`)
#[derive(Debug)]
enum Status {
    Good,
    Bad(&'static str),
}

// --------

/// ## Args Enum
///
/// An enum for the command line arguments.
///
/// - `UserArgs(Vec<String>)` - Command line arguments vector; user variable (passed to function argument)
/// - `SysArgs`               - Command line arguments vector; "system" variable
enum Args {
    UserArgs(Vec<String>),
    SysArgs,
}

// ----------------

// __ Functions __

/// ## check_args Function
///
/// Checks the command line arguments passed to the compiler command.
///
/// ### Parameters
/// `args: &Args` - The arguments to check (optional)
///
/// ### Returns
/// #### Return Type
/// `Result<Status, &'static str>`
///
/// #### Return Values
/// `OK(Status::Good)`
/// `OK(Status::Bad("Unrecognised arguments: " + &args[1..args.len() - 1]))`
///
/// ### Body
/// ```
/// if args == Args::SysArgs {
///     let args = env::args().collect();
/// }
///
/// match args[1].as_str() {
///     ArgOption::Version => {
///         println!("{}", VERSION);
///         Ok(Status::Good)
///     },
///     ArgOption::Help => {
///         println!("{}", ARGS_HELP);
///         Ok(Status::Good)
///     },
/// }
/// ```

// --------

/// ## args_help Function
///
/// Returns the help string for the compiler command.
///
/// ### Parameters
/// -
///
/// ### Returns
/// #### Return Type
/// String
///
/// #### Return Values
/// \*Too Long
///
/// ### Body
/// ```
/// let help_intro_string: String = String::from(
///     "\n`kiracode` command - help documentation\nVersion 0\n\n"
/// );
/// let help_outro_string: String = String::from(
///     "For more specific help on a command, use `kiracode --help <command>`\nTo open online documentation, use flag `--online` alongside `--help` flag\n"
/// );
///
/// let final_string: String = help_intro_string + option_args_help() + sub_commands_help() + help_outro_string;
///
/// final_string
/// ```
fn args_help() -> String {}

// ----------------

// __ MAIN __

/// ## main -> () Function
///
/// The main function of the compiler program.
fn main() {
    // __ Constants __
    pub const VERSION: i32 = 0;
    pub const ARGS_HELP: String = args_help();

    // &args[0] is the path of the file or the name of the command
    let args: Vec<String> = env::args().collect();

    dbg!(args);
}
