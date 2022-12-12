fn day_fn(input: &str) -> u32 {
    0
}

fn main() {
    println!("input: {}", day_fn(include_str!("input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(day_fn(include_str!("test.txt")), 0);
    }
}
