
pub fn run() -> String {

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn challenge_correct() {
        let result = run();
        assert_eq!(
            result,
            "746865206b696420646f6e277420706c6179"
        )
    }

}