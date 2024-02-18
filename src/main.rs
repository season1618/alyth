use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_path = &args[1];

    let Ok(code) = fs::read_to_string(src_path) else {
        println!("could not open the source file.");
        return;
    };

    println!("{}", code);
}
