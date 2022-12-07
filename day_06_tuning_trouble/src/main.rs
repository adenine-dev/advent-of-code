use std::collections::HashSet;

fn first_start_of_marker(input: &str, unique_chars: usize) -> usize {
    'outer: for (idx, slice) in input.as_bytes().windows(unique_chars).enumerate() {
        let mut hs = HashSet::new();
        for c in slice {
            if !hs.insert(c) {
                continue 'outer;
            }
        }
        return idx + unique_chars;
    }
    0
}

fn main() {
    println!(
        "first start of packet occurs at {}",
        first_start_of_marker(include_str!("input.txt"), 4)
    );
    println!(
        "first start of message occurs at {}",
        first_start_of_marker(include_str!("input.txt"), 14)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            first_start_of_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            7
        );
        assert_eq!(first_start_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(first_start_of_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            first_start_of_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            first_start_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );

        assert_eq!(
            first_start_of_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
        assert_eq!(
            first_start_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            23
        );
        assert_eq!(
            first_start_of_marker("nppdvjthqldpwncqszvftbrmjlhg", 14),
            23
        );
        assert_eq!(
            first_start_of_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            first_start_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
