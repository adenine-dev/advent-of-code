fn calorie_count(input: &str, top_n: usize) -> Result<u32, &'static str> {
    let mut calories = input
        .split("\n\n")
        .map(|x| (x.split('\n').flat_map(|s| s.parse::<u32>()).sum::<u32>()))
        .collect::<Vec<_>>();
    calories.sort();

    Ok(calories.iter().rev().take(top_n).sum::<u32>())
}

fn main() {
    println!(
        "most calories carried by 1 elf: {}",
        calorie_count(include_str!("input.txt"), 1).unwrap()
    );

    println!(
        "sum of calories carried by the top 3 elves: {}",
        calorie_count(include_str!("input.txt"), 3).unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            calorie_count(include_str!("test.txt"), 1).expect("failed: "),
            24000
        );

        assert_eq!(
            calorie_count(include_str!("test.txt"), 3).expect("failed: "),
            45000
        );
    }
}
