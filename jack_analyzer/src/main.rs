mod jack_tokenizer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_name = &args[1];

    if is_file(target_name) {
        println!("is file");
    } else {
        println!("is not file");
    }
}

fn is_file(target_name: &str) -> bool {
    target_name.contains(".jack")
}
