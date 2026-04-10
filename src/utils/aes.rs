use aes::Aes128;
use aes::cipher::consts::U16;
use aes::cipher::{Array, BlockCipherDecrypt, KeyInit};

pub fn decrypt_aes_128_ecb(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher: Aes128 = Aes128::new(&Array::try_from(key).unwrap());

    bytes
        .chunks(16)
        .flat_map(|chunk: &[u8]| {
            let mut block: Array<u8, U16> = Array::try_from(chunk).unwrap();
            cipher.decrypt_block(&mut block);
            block.to_vec()
        })
        .collect()
}

pub fn detect_aes_128_ecb(bytes: &[u8]) -> bool {
    let chunks: Vec<&[u8]> = bytes.chunks(16).collect();

    for i in 0..chunks.len() {
        for j in (i + 1)..chunks.len() {
            if chunks[i] == chunks[j] {
                return true;
            }
        }
    }

    false
}
