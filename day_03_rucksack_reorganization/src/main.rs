#![feature(int_log)]

fn ascii_to_priority(c: u8) -> u64 {
    let i = (c) ^ 0b1100000;
    (if i > 26 { i - 6 } else { i }) as u64
}

fn rucksack_reorganization(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let masks = [first, second].map(|s| {
                let mut mask = 0u64;
                s.as_bytes().iter().for_each(|c| {
                    mask |= 1 << ascii_to_priority(*c);
                });
                mask
            });

            (masks[0] & masks[1]).ilog2()
        })
        .sum::<u32>()
}

fn main() {
    println!(
        "sum of priorities of mixed item is: {}",
        rucksack_reorganization(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rucksack_reorganization(include_str!("test.txt")), 157);
    }
}
