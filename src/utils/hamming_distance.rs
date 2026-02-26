pub fn calculate_hamming_distance(bytes: &[u8], bytes2: &[u8]) -> u32 {
    bytes
        .iter()
        .zip(bytes2)
        .map(|(&b, &b2)| (b ^ b2).count_ones())
        .sum()
}
