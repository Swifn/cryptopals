pub fn scoring(s: &str) -> i32 {
    s.chars()
        .map(|c: char| match c.to_ascii_lowercase() {
            'e' | 't' | 'a' | 'o' | 'i' => 2,
            'a'..='z' => 1,
            ' ' => 3,
            _ => -1,
        })
        .sum()
}
