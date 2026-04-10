use crate::utils::aes::detect_aes_128_ecb;
use crate::utils::encoding::hex_to_bytes;
use std::fs::File;
use std::io::Read;

pub fn run() -> String {
    let mut f: File = File::open("data/set1_challenge8.txt").expect("Failed to open file.");
    let mut data: Vec<u8> = vec![];
    (f).read_to_end(&mut data).expect("Failed to read file");

    let lines: Vec<String> = String::from_utf8_lossy(&data)
        .lines()
        .map(String::from)
        .collect();

    let line_bytes: Vec<Vec<u8>> = lines
        .iter()
        .map(|line: &String| hex_to_bytes(line))
        .collect();

    for (i, line) in line_bytes.iter().enumerate() {
        if detect_aes_128_ecb(line) {
            // println!("ECB detected on line {}: {}", i, lines[i]);
            return lines[i].clone();
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_correct() {
        let result: String = run();
        assert_eq!(
            result,
            "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a"
        )
    }
}
