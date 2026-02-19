pub fn bytes_to_hex(s: &str) -> Vec<u8> {
    unimplemented!()
}

pub fn hex_to_bytes(s: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    for i in (0..chars.len()).step_by(2) {
        let hi: u32 = chars[i].to_digit(16).unwrap();
        let lo: u32 = chars[i + 1].to_digit(16).unwrap();

        let res: u8 = (hi << 4 | lo) as u8;

        bytes.push(res);
    }

    bytes
}

pub fn bytes_to_base64(v: Vec<u8>) -> String {
    unimplemented!()
}
