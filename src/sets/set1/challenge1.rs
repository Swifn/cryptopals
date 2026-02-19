use crate::utils::encoding::bytes_to_base64;
use crate::utils::encoding::hex_to_bytes;

pub fn run() -> String {
    let hex_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let byte_vec: Vec<u8> = hex_to_bytes(hex_string);
    let base64_string: String = bytes_to_base64(&byte_vec);

    base64_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_correct() {
        let result = run();
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        )
    }
}
