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
