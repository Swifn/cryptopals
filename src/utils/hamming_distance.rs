pub struct Hamming {
    pub key_size: usize,
    pub normalised_distance: f32,
}

pub fn calculate_hamming_distance(bytes: &[u8], bytes2: &[u8]) -> u32 {
    bytes
        .iter()
        .zip(bytes2)
        .map(|(&b, &b2)| (b ^ b2).count_ones())
        .sum()
}

pub fn estimate_key_size(min_range: u8, max_range: u8, data: &[u8], chunk_amount: u8) -> Hamming {
    (min_range..=max_range)
        .map(|key_size| {
            let normalised_distance = data
                .chunks(key_size as usize)
                .take(chunk_amount as usize)
                .collect::<Vec<_>>()
                .windows(2)
                .map(|w| calculate_hamming_distance(w[0], w[1]) as f32 / key_size as f32)
                .sum::<f32>()
                / (chunk_amount.saturating_sub(1) as f32);

            println!("KEY: {} DISTANCE: {}", &key_size, &normalised_distance);

            Hamming {
                key_size: key_size as usize,
                normalised_distance,
            }
        })
        .min_by(|a, b| {
            a.normalised_distance
                .partial_cmp(&b.normalised_distance)
                .unwrap()
        })
        .unwrap()
}
