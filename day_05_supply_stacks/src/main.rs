fn parse_start(start: &str) -> Vec<Vec<char>> {
    let mut start_iter = start.lines().rev();
    let mut stacks = vec![vec![]; (start_iter.next().unwrap().len() + 1) / 4];
    for layer in start_iter {
        layer
            .as_bytes()
            .chunks(4)
            .flat_map(std::str::from_utf8)
            .enumerate()
            .for_each(|(idx, s)| {
                if !s.trim().is_empty() {
                    stacks[idx].push(s.as_bytes()[1] as char);
                }
            });
    }
    stacks
}

fn move_crates(port: &mut [Vec<char>], amt: usize, start: usize, end: usize) {
    for _ in 0..amt {
        let x = port[start].pop().unwrap();
        port[end].push(x);
    }
}

fn rearrange_crates(input: &str) -> String {
    let (start, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_start(start);

    commands.lines().for_each(|line| {
        let [amt, start, end] = line
            .split(' ')
            .flat_map(|s| s.parse::<usize>())
            .collect::<Vec<_>>()[..]
        else { panic!("invalid input") };
        move_crates(&mut stacks, amt, start - 1, end - 1);
    });

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    println!(
        "top crates after moving: {}",
        rearrange_crates(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rearrange_crates(include_str!("test.txt")), "CMZ".to_owned());
    }
}
