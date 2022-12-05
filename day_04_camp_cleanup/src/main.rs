fn count_fully_contained(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let [a1, a2, b1, b2] = &line
                .split([',', '-'])
                .flat_map(|num| num.parse::<u32>())
                .collect::<Vec<_>>()[..] 
            else { panic!("invalid input") };

            u32::from((a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2))
        })
        .sum()
}

fn count_overlapping(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let [a1, a2, b1, b2] = &line
                .split([',', '-'])
                .flat_map(|num| num.parse::<u32>())
                .collect::<Vec<_>>()[..] 
            else { panic!("invalid input") };

            u32::from(a1 <= b2 && b1 <= a2)
        })
        .sum()
}


fn main() {
    println!(
        "number of pairs where one fully contains the other: {}",
        count_fully_contained(include_str!("input.txt"))
    );
    
    println!(
        "number of pairs that overlap: {}",
        count_overlapping(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_fully_contained(include_str!("test.txt")), 2);
        assert_eq!(count_overlapping(include_str!("test.txt")), 4);
    }
}
