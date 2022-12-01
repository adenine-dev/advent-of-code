use std::error::Error;

fn calorie_count(input: &str) -> Result<u32, &'static str> {
    input
        .split("\n\n")
        .map(|x| (x.split('\n').flat_map(|s| s.parse::<u32>()).sum::<u32>()))
        .max()
        .ok_or("invalid input.")
}

fn main() {
    println!("{}", calorie_count(include_str!("input.txt")).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calorie_count(include_str!("test.txt"),).expect("failed: "),
            24000
        );
    }
}
