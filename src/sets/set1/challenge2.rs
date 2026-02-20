use crate::utils::encoding::bytes_to_hex;
use crate::utils::encoding::hex_to_bytes;
use crate::utils::xor::fixed_xor;
pub fn run() -> String {
    let s: &str = "1c0111001f010100061a024b53535009181c";
    let key: &str = "686974207468652062756c6c277320657965";

    let s_bytes: Vec<u8> = hex_to_bytes(s);
    let key_bytes: Vec<u8> = hex_to_bytes(key);
    let xor: Vec<u8> = fixed_xor(&s_bytes, &key_bytes);
    let result: String = bytes_to_hex(&xor);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_correct() {
        let result: String = run();
        assert_eq!(result, "746865206b696420646f6e277420706c6179")
    }
}
