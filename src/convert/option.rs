pub struct OptionAction {
    pub base_type: u8,
    pub file_path: String,
    pub key: Option<String>,
    pub is_error: bool,
}

impl OptionAction {
    pub fn get_filename(&self) -> String {
        self.file_path
            .split("/")
            .last()
            .expect("Not found file name")
            .to_string()
    }
    pub fn parse(argv: &Vec<String>) -> Self {
        let mut argv = argv.clone();
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
}
