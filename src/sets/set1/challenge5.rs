use crate::utils::encoding::bytes_to_hex;
use crate::utils::xor::repeating_key_xor;

pub fn run() -> String {
    let s: &str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";

    let hex = bytes_to_hex(&repeating_key_xor(s.as_bytes(), b"ICE"));

    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_correct() {
        let result: String = run();
        assert_eq!(
            result,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }
}
