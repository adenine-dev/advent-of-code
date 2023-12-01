fn rock_paper_scissors_rps(input: &str) -> u32 {
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

fn rock_paper_scissors_wld(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (other, me) = line.split_once(' ').unwrap();
            let outcome = match me {
                "X" => 0, // lose
                "Y" => 1, // draw
                "Z" => 2, // win
                &_ => unreachable!(),
            };

            let other_code = match other {
                "A" => 0, // rock
                "B" => 1, // paper
                "C" => 2, // scissors
                &_ => unreachable!(),
            };

            let win_score = outcome * 3;
            let shape_score = (other_code + outcome) % 3;
            let shape_score = if shape_score == 0 { 3 } else { shape_score };
            shape_score + win_score
        })
        .sum()
}

fn main() {
    println!(
        "projected score if opponents moves are encoded: {}",
        rock_paper_scissors_rps(include_str!("input.txt"))
    );

    println!(
        "projected score if outcome is encoded: {}",
        rock_paper_scissors_wld(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rock_paper_scissors_rps(include_str!("test.txt")), 15);
        assert_eq!(rock_paper_scissors_wld(include_str!("test.txt")), 12);
    }
}
