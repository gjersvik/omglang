use clap::{App, Arg};
use std::fs;

use omglang::run;

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

    let source =
        fs::read_to_string(matches.value_of("SRC_FILE").unwrap()).expect("could not read file");
    run(&source);
}
