pub fn fixed_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xor: Vec<u8> = Vec::new();

    for i in 0..bytes.len() {
        xor.push(bytes[i] ^ key[i])
    }

    xor
}
