use crate::utils::block_manipulation::incrementing_block_transposer;
use crate::utils::encoding::base64_to_bytes;
use crate::utils::hamming_distance::{Hamming, estimate_key_size};
use std::fs;

pub fn run() -> String {
    let f: String = fs::read_to_string("data/set1_challenge6.txt").expect("Failed to open file");

    let bytes: Vec<u8> = base64_to_bytes(f);

    let key_size: Hamming = estimate_key_size(2, 40, &bytes, 10);

    let result: String = String::from_utf8(incrementing_block_transposer(
        &bytes,
        key_size.key_size as u8,
    ))
    .unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_correct() {
        let result: String = run();
        assert_eq!(result, "Terminator X: Bring the noise")
    }
}
