pub struct OptionEncript {
    pub key: Result<String, ()>,
    pub file_path: String,
}

impl OptionEncript {
    pub fn get_filename(&self) -> String {
        self.file_path
            .split("/")
            .last()
            .expect("Not found file name")
            .to_string()
    }

    pub fn parse(argv: &Vec<String>) -> Self {
        let mut argv = argv.clone();
        let argv_len = argv.len();
        let mut key: Result<String, ()> = Err(());
        let mut index_to_remove: Vec<usize> = vec![];

        for i in 2..argv_len {
            if argv[i] == "-k" || argv[i] == "--key" {
                if let Some(v) = argv.get(i + 1) {
                    key = Ok(v.to_string());
                    index_to_remove.push(i + 1);
                }
                index_to_remove.push(i);
            }
        }
        for idx in index_to_remove {
            argv.remove(idx);
        }

        let file_path = argv.get(2).expect("Error file name").to_string();

        Self {
            key: key,
            file_path: file_path,
        }
    }
}
