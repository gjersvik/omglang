#![warn(clippy::all)]
use clap::{App, Arg};
use tokio::prelude::Future;

use omglang::OmgLang;

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

    let omg = OmgLang::new();
    let future = omg
        .run_file(matches.value_of("SRC_FILE").unwrap())
        .map_err(|e| eprint!("{}", e));
    tokio::run(future);
}
