use crate::utils::scoring::scoring;

pub struct Score {
    pub text: String,
    pub key: u8,
    pub score: i32,
}

pub fn fixed_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {

    bytes.iter().zip(key).map(|(&b, &k)| b ^ k ).collect()

}

pub fn single_byte_xor_cipher(bytes: &[u8]) -> Score {
    let mut best: Option<Score> = None;

    for key in 0u8..=255u8 {

        let xor: Vec<u8> = bytes.iter().map(|&b| b ^ key).collect();

        if let Ok(text) = String::from_utf8(xor){
            let sc: i32 = scoring(&text);
            let candidate = Score {text, key, score: sc};

            if best.as_ref().map_or(true, | b| candidate.score > b.score){
                best = Some(candidate)
            }
        }
    }

    best.unwrap()
}
