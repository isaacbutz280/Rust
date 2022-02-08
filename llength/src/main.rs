/* Take 2 args, a file name and a number. Iterate over each line,
 and if the length is longer than the num, print it out */

use llength::config::Config;

use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = llength::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
