pub fn to_base_string(file: &Vec<u8>, base: u8) -> String {
    let mut str_bytes = String::new();
    for b in file {
        if base == 2 {
            str_bytes += format!(" {:b}", b).as_str();
        } else if base == 8 {
            str_bytes += format!(" {:o}", b).as_str();
        } else if base == 16 {
            str_bytes += format!(" {:x}", b).as_str();
        }
    }
    str_bytes
}

