use crate::utils::xor::single_byte_xor_cipher;

pub fn incrementing_block_transposer(bytes: &[u8], key_size: u8) -> Vec<u8> {
    let chunks: Vec<&[u8]> = bytes.chunks(key_size as usize).collect();

    let transposed: Vec<Vec<u8>> = (0..key_size as usize)
        .map(|i: usize| {
            chunks
                .iter()
                .filter(|chunk: &&&[u8]| i < chunk.len())
                .map(|chunk: &&[u8]| chunk[i])
                .collect()
        })
        .collect();

    transposed
        .iter()
        .map(|block: &Vec<u8>| single_byte_xor_cipher(block).key)
        .collect()
}
