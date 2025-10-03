// cargo run --bin grep match ./src/bin/grep/main.rs
mod grep;

use structopt::StructOpt;
// use std::env::args;
use grep::run;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    # [structopt(name = "PATTERN")]
    pattern: String,
    # [structopt(name = "FILE")]
    path: Vec<String>,
}

impl GrepArgs {
    // from_args関数を使用することで不要になった
    // fn new(path: String, pattern: String) -> GrepArgs {
    //     GrepArgs {path, pattern}
    // }
}


fn main() {
    // let pattern = args().nth(1);
    // let path = args().nth(2);
    // match (pattern, path){
    //     (Some(pattern),Some(path)) => run(GrepArgs::new(path, pattern)),
    //     _ => println!("pattern or path is not specified!!")
    // }

    run(GrepArgs::from_args());
}