fn first_start_of_packet(input: &str) -> usize {
    for (idx, slice) in input.as_bytes().windows(4).enumerate() {
        if slice[0] != slice[1]
            && slice[0] != slice[2]
            && slice[0] != slice[3]
            && slice[1] != slice[2]
            && slice[1] != slice[3]
            && slice[2] != slice[3]
        {
            return idx + 4;
        }
    }
    0
}

fn main() {
    println!(
        "first start of packet occurs at {}",
        first_start_of_packet(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(first_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(first_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(first_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(
            first_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(
            first_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );
    }
}
