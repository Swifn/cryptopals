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
    const BASE64_TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes: Vec<u8> = v;
    let mut out: String = String::new();

    let mut i = 0;
    while i < bytes.len() {
        let b1: u8 = bytes[i];
        let b2: u8 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b3: u8 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        let triple: u32 = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        let c1 = ((triple >> 18) & 0b111111) as usize;
        let c2 = ((triple >> 12) & 0b111111) as usize;
        let c3 = ((triple >> 6) & 0b111111) as usize;
        let c4 = (triple & 0b111111) as usize;

        out.push(BASE64_TABLE[c1] as char);
        out.push(BASE64_TABLE[c2] as char);

        if i + 1 < bytes.len() {
            out.push(BASE64_TABLE[c3] as char)
        } else {
            out.push('=');
        }

        if i + 2 < bytes.len() {
            out.push(BASE64_TABLE[c4] as char)
        } else {
            out.push('=');
        }

        i += 3
    }

    println!("{}", out);

    out
}
