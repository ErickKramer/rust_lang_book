use std::env;
use std::process;

use minigrep::Config;


fn main() {
    // std::env::args will panic if any arguments contains invalid Unicode
    // The output of args is an iterator that gets converted into a collection via the collect
    // method
    let args: Vec<String> = env::args().collect();
    // If an error occurs it runs the code in the anonymous function (aka closure)
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // we use `if let` as we are not interested in the success output of the run method, but in the
    // presence of an error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
