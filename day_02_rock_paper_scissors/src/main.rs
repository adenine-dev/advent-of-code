fn rock_paper_scissors(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (other, me) = line.split_once(' ').unwrap();
            let shape_score = match me {
                "X" => 1, // rock
                "Y" => 2, // paper
                "Z" => 3, // scissors
                &_ => unreachable!(),
            };

            let other_code = match other {
                "A" => 0, // rock
                "B" => 2, // paper
                "C" => 1, // scissors
                &_ => unreachable!(),
            };

            let win_score = ((shape_score + other_code) % 3) * 3;
            shape_score + win_score
        })
        .sum()
}

fn main() {
    println!(
        "projected score: {}",
        rock_paper_scissors(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rock_paper_scissors(include_str!("test.txt")), 15);
    }
}
