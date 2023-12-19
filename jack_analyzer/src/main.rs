mod jack_tokenizer;

use std::{env, ops::Deref, process};

fn main() {
    let target_name = get_target();

    if is_file(&target_name) {
        println!("is file");
    } else {
        println!("is not file");
    }
}

fn get_target() -> String {
    let args: Vec<String> = env::args().collect();
    let target_name = args.get(1).unwrap_or_else(|| {
        eprintln!("not enough argumens");
        process::exit(1);
    });
    String::from(target_name)
}

fn is_file(target_name: &str) -> bool {
    target_name.contains(".jack")
}
