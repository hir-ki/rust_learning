use std::fs::read_to_string;
use crate::GrepArgs;

fn grep(content: String, state: &GrepArgs, file_name: &str) {
    for line in content.lines() {
        if line.contains(state.pattern.as_str()) {
            println!("{}: {}",file_name, line);
        }
    }
}

// pub fn run (state: GrepArgs){
//     for file in state.path.iter() {

//         match read_to_string(file){
//             Ok(content) => grep(content, &state, file),
//             Err(reason) => println!("{}",reason),
//         }
//     }
// }

// // 関数型的アプローチ
// pub fn run (state: GrepArgs) {
//     state.path.iter().for_each(|file| match read_to_string(file) {
//         Ok(content) => grep(content, &state, &file),
//         Err(reason) => print!("{}",reason),
//     });
// }

// iteratorを並列で回す
use rayon::prelude::*;
pub fn run (state: GrepArgs) {
    state.path.par_iter().for_each(|file| match read_to_string(file) {
        Ok(content) => grep(content, &state, &file),
        Err(reason) => print!("{}",reason),
    });
}