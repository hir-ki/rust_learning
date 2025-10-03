use std::fs::read_to_string;

pub fn run_cat(path: String) {
    match read_to_string(path){
        Ok(content) => println!("{}", content),
        Err(reason) => println!("reason:{}", reason),
    }
}