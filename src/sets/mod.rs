pub mod set1;

use std::collections::HashMap;

pub type ChallengeFn = fn() -> String;

pub fn get_challenges() -> HashMap<(&'static str, &'static str), ChallengeFn> {
    let mut map: HashMap<(&str, &str), ChallengeFn> = HashMap::new();

    map.insert(("set1", "1"), set1::challenge1::run);
    map.insert(("set1", "2"), set1::challenge2::run);
    // map.insert(("set1", "3"), set1::challenge3::run);
    // map.insert(("set1", "4"), set1::challenge4::run);
    // map.insert(("set1", "5"), set1::challenge5::run);

    map
}
