use std::fs;

use super::{decript, encript, OptionEncript};

pub fn encript_file(argv: &Vec<String>) {
    let opt = OptionEncript::parse(argv);
    if opt.key == Err(()) {
        println!("Error not found key");
        return;
    }

    let content = fs::read(opt.file_path.clone()).expect("Error read file");
    let content = encript(&content, opt.key.unwrap().as_str());

    fs::write(opt.file_path.clone(), content).expect("Error cannot write file");
    println!("\nencript {} successfuly.", opt.file_path);
}

pub fn decript_file(argv: &Vec<String>) {
    let opt = OptionEncript::parse(argv);
    if opt.key == Err(()) {
        println!("Error not found key");
        return;
    }

    let content = fs::read(opt.file_path.clone()).expect("Error read file");
    let content = decript(&content, opt.key.unwrap().as_str());

    match content {
        Ok(byte) => {
            fs::write(opt.file_path.clone(), byte).expect("Error cannot write file");
            println!("\ndecript {} successfuly.", opt.file_path);
        }
        Err(_) => {
            println!("Key is not correct");
        }
    }
}
