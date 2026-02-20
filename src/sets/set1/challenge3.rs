use crate::utils::encoding::hex_to_bytes;
use crate::utils::xor::{Score, single_byte_xor_cipher};
pub fn run() -> String {
    let s: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let s_bytes: Vec<u8> = hex_to_bytes(s);
    let result: Score = single_byte_xor_cipher(&s_bytes);

    println!(
        "INDEX: {} KEY: | {} | SCORE: {} TEXT: {}",
        result.key, result.key as char, result.score, result.text
    );

    result.text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_correct() {
        let result: String = run();
        assert_eq!(result, "Cooking MC's like a pound of bacon")
    }
}
