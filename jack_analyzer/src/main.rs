mod jack_tokenizer;

use std::{env, ops::Deref, process};

use jack_tokenizer::JackTokenizer;

fn main() {
    let target_name = get_target();

    let tokenizer = JackTokenizer::new(&target_name).unwrap_or_else(|_| {
        eprintln!("{} does not exsit", target_name);
        process::exit(1);
    });

    tokenizer.create_token_xml_file(&(target_name.replace(".jack", "") + "_token.xml"));
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
