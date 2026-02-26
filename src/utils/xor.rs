use crate::utils::scoring::scoring;

pub struct Score {
    pub text: String,
    pub key: u8,
    pub score: i32,
}

pub fn fixed_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    bytes.iter().zip(key).map(|(&b, &k)| b ^ k).collect()
}

pub fn single_byte_xor_cipher(bytes: &[u8]) -> Score {
    (0u8..=255)
        .map(|key: u8| {
            let xor: Vec<u8> = bytes.iter().map(|b: &u8| b ^ key).collect();
            let text: String = String::from_utf8_lossy(&xor).to_string();
            let sc: i32 = scoring(&text);

            Score {
                text,
                key,
                score: sc,
            }
        })
        .max_by_key(|s: &Score| s.score)
        .unwrap()
}
