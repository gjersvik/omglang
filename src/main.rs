#![warn(clippy::all)]
use clap::{App, Arg};

use omglang::run_file;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let matches = App::new("O.M.G. Language")
        .version("0.0.0")
        .author("Ole Martin Gjersvik")
        .about("The multi core language.")
        .arg(
            Arg::with_name("SRC_FILE")
                .help("the .omg file to run")
                .required(true)
                .index(1),
        )
        .get_matches();

    match run_file(matches.value_of("SRC_FILE").unwrap()) {
        Ok(_) => (),
        Err(err) => eprint!("{}", err),
    };
}
