pub mod compilation_engine;
pub mod jack_tokenizer;

use std::{
    env,
    fs::{self, File},
    process,
};

use compilation_engine::CompilationEngine;
use jack_tokenizer::JackTokenizer;

fn main() {
    let target_name = get_target();
    let mut jack_files = Vec::new();

    if is_file(&target_name) {
        jack_files.push(target_name.clone())
    } else {
        jack_files = get_target_type_files(&target_name, "jack");
    }

    for jack_file in jack_files {
        let mut tokenizer = JackTokenizer::new(&jack_file).unwrap_or_else(|_| {
            eprintln!("{} does not exsit", jack_file);
            process::exit(1);
        });
        let mut compilation_engine = CompilationEngine::new(&jack_file).unwrap_or_else(|_| {
            eprintln!("{} does not exsit", jack_file);
            process::exit(1);
        });
        let _ = tokenizer.write_token_file(&jack_file);
        let _ = compilation_engine.compile_class();
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

fn get_target_type_files(dir_name: &str, target_extension: &str) -> Vec<String> {
    let entries = fs::read_dir("./".to_string() + dir_name).unwrap();

    let mut jack_files = Vec::<String>::new();

    for entry in entries {
        jack_files.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    jack_files
        .iter()
        .filter(|&file_name| file_name.clone().contains(target_extension))
        .cloned()
        .collect()
}

fn is_file(target_name: &str) -> bool {
    target_name.contains(".jack")
}
