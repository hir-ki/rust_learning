// we can run this command. cargo run --bin cat

use std::env::args;
mod run_cat;

use run_cat::run_cat;


fn main() {
    // Pattern1
    // match args().nth(1) {
    //     Some(path) => run_cat(path),
    //     None => println!(" No path is specified! "),
    // }

    // Pattern2
    if let Some(path) = args().nth(1) {
        run_cat(path)
    }
}