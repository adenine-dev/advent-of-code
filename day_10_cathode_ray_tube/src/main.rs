fn signal_strength(input: &str, cycles: &[u32]) -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    input
        .lines()
        .flat_map(|line| {
            let (command, arg) = line.split_at(4);
            match command {
                "noop" => {
                    cycle += 1;
                    [
                        if cycles.contains(&(cycle - 1)) {
                            Some(x)
                        } else {
                            None
                        },
                        None,
                    ]
                }
                "addx" => [1, 2].map(|i| {
                    let ret = if cycles.contains(&cycle) {
                        Some(x)
                    } else {
                        None
                    };

                    cycle += 1;
                    if i == 2 {
                        x += arg.trim().parse::<i32>().unwrap();
                    }

                    ret
                }),
                _ => panic!("invalid command :<"),
            }
        })
        .flatten()
        .zip(cycles)
        .fold(0, |acc, (x, c)| acc + x * *c as i32)
}

fn main() {
    println!(
        "sum of signal strength at given cycles: {}",
        signal_strength(include_str!("input.txt"), &[20, 60, 100, 140, 180, 220])
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            signal_strength(include_str!("test.txt"), &[20, 60, 100, 140, 180, 220]),
            13140
        );
    }
}
