use crate::utils::encoding::hex_to_bytes;
use crate::utils::xor::{Score, single_byte_xor_cipher};
use std::fs::File;
use std::io::Read;

pub fn run() -> String {
    let mut f: File = File::open("data/set1_challenge4.txt").expect("Failed to open file.");
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).expect("Failed to read file");

    let lines: Vec<String> = String::from_utf8_lossy(&data)
        .lines()
        .map(String::from)
        .collect();

    let result: Score = lines
        .iter()
        .map(|line: &String| single_byte_xor_cipher(&hex_to_bytes(line)))
        .max_by_key(|s: &Score| s.score)
        .unwrap();

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
        let result = run();
        assert_eq!(result, "Now that the party is jumping")
    }
}
