use crate::{
    convert_byte::to_base_string,
    encryption::{self, main::decript},
};
use std::fs::{self};

struct OptionAction {
    base_type: u8,
    file_path: String,
    key: Option<String>,
    is_error: bool,
}

fn split_option(argv: &mut Vec<String>) -> OptionAction {
    let mut aoption = OptionAction {
        base_type: 16,
        file_path: "".to_string(),
        key: None,
        is_error: false,
    };
    let argv_len = argv.len();
    let mut index_to_remove: Vec<usize> = vec![];
    for i in 2..argv_len {
        let command = argv.get(i).expect("error argv").to_string();
        if command == "--key" || command == "-k" {
            aoption.key = Some(argv.get(i + 1).expect("error not found key").to_string());
            index_to_remove.push(i);
            index_to_remove.push(i + 1);
        }
        if command == "--base" || command == "-b" {
            // get base value
            aoption.base_type = argv
                .get(i + 1)
                .expect("error in `file_to_bytes` find value of base argv")
                .parse::<u8>()
                .expect("error at `file_to_bytes` convert argv to string");
            // check base value
            if aoption.base_type != 2 && aoption.base_type != 8 && aoption.base_type != 16 {
                println!("Error base type [2, 8, 16]");
                aoption.is_error = true;
            }
            index_to_remove.push(i);
            index_to_remove.push(i + 1);
        }
    }
    index_to_remove.sort();
    index_to_remove.reverse();
    for i in index_to_remove {
        argv.remove(i);
    }
    aoption.file_path = argv.get(2).expect("error path file").to_string();
    aoption
}

pub fn file_to_bytes(argv: &mut Vec<String>) {
    let options = split_option(argv);
    if options.is_error {
        return;
    }
    // write file name in first position of text file.
    let file_name: String = (options
        .file_path
        .split("/")
        .last()
        .expect("not found file name"))
    .to_string();
    let mut byte_string: String = file_name.clone();
    // save base type to text file.
    byte_string += format!(" {}", options.base_type).as_str();
    let mut content = fs::read(options.file_path).expect("error read file");
    if let Some(v) = options.key {
        content = encryption::main::encript(&content, &v);
    }
    byte_string += to_base_string(&content, options.base_type).as_str();
    fs::write(format!("./{}.txt", file_name), byte_string).expect("cannot write file");

    // report status
    println!("\nConvert file to base {} successfuly.", options.base_type);
    println!("output ./{}.txt", file_name);
}

pub fn bytes_to_file(argv: &mut Vec<String>) {
    let mut options = split_option(argv);

    let content = fs::read_to_string(options.file_path).expect("error read file");
    let mut content_split = content.split(" ").collect::<Vec<&str>>();
    let file_name = *content_split.get(0).expect("Error read file name");
    options.base_type = content_split
        .get(1)
        .expect("Error read base type")
        .parse::<u8>()
        .expect("Error read base type");
    content_split.remove(1); // remove base type
    content_split.remove(0); // remove file name

    let mut content_byte = content_split
        .iter()
        .map(|f| {
            u8::from_str_radix(f, options.base_type as u32).expect("Error convert string to number")
        })
        .collect::<Vec<u8>>();
    if let Some(v) = options.key {
        if let Ok(byte) = decript(&content_byte, &v) {
            content_byte = byte;            
        }
   }

    fs::write(file_name, content_byte).expect("cannot write file");
    println!("\nConvert base {} to file successfuly.", options.base_type);
    println!("output ./{}", file_name);
}
