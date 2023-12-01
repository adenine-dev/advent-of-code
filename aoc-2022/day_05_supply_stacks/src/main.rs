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

fn rearrange_crates_9000(input: &str) -> String {
    let (start, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_start(start);

    commands.lines().for_each(|line| {
        let [amt, start, end] = line
            .split(' ')
            .flat_map(|s| s.parse::<usize>())
            .collect::<Vec<_>>()[..]
        else { panic!("invalid input") };
        for _ in 0..amt {
            let x = stacks[start - 1].pop().unwrap();
            stacks[end - 1].push(x);
        }
    });

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn rearrange_crates_9001(input: &str) -> String {
    let (start, commands) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_start(start);

    commands.lines().for_each(|line| {
        let [amt, start, end] = line
            .split(' ')
            .flat_map(|s| s.parse::<usize>())
            .collect::<Vec<_>>()[..]
        else { panic!("invalid input") };
        let taken = stacks[start - 1].len() - amt;
        let mut x = stacks[start - 1].split_off(taken);
        stacks[end - 1].append(&mut x);
    });

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    println!(
        "top crates after moving using the CrateMover 9000™: {}",
        rearrange_crates_9000(include_str!("input.txt"))
    );

    println!(
        "top crates after moving using the CrateMover 9001™: {}",
        rearrange_crates_9001(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            rearrange_crates_9000(include_str!("test.txt")),
            "CMZ".to_owned()
        );

        assert_eq!(
            rearrange_crates_9001(include_str!("test.txt")),
            "MCD".to_owned()
        );
    }
}
