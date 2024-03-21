use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn encript(content: &Vec<u8>, key: &str) -> Vec<u8> {
    let mc = new_magic_crypt!(key, 256);
    let content_encrypt = mc.encrypt_bytes_to_bytes(&content);
    return content_encrypt.to_owned();
}

pub fn decript(content: &Vec<u8>, key: &str) -> Result<Vec<u8>, Vec<u8>> {
    let mc = new_magic_crypt!(key, 256);
    match mc.decrypt_bytes_to_bytes(&content) {
        Ok(v) => Ok(v),
        Err(_) => Err(vec![0]),
    }
}
