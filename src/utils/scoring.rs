pub fn scoring(s: &str) -> i32 {

    s.chars().map(|c:char | match c {
        'e' | 't' | 'a' | 'o' | 'i' => 2,
        'a'..='z' => 1,
        '_' => 3,
        _ => 1,
    }).sum()
}
