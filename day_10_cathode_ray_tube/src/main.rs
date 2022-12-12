#![feature(iter_intersperse)]

const CYCLES: [u32; 6] = [20, 60, 100, 140, 180, 220];

fn signal_strength(input: &str) -> i32 {
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
                        if CYCLES.contains(&(cycle - 1)) {
                            Some(x)
                        } else {
                            None
                        },
                        None,
                    ]
                }
                "addx" => [1, 2].map(|i| {
                    let ret = if CYCLES.contains(&cycle) {
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
        .zip(CYCLES)
        .fold(0, |acc, (x, c)| acc + x * c as i32)
}

fn draw_crt(input: &str) {
    let mut cycle = 0;
    let mut x = 1;
    let draw = |cycle: i32, x: i32| {
        if ((cycle % 40) - x).abs() <= 1 {
            '#'
        } else {
            ' '
        }
    };
    println!(
        "{}",
        input
            .lines()
            .flat_map(|line| {
                let (command, arg) = line.split_at(4);
                match command {
                    "noop" => {
                        cycle += 1;
                        [None, Some(draw(cycle - 1, x))]
                    }
                    "addx" => [1, 2].map(|i| {
                        let ret = Some(draw(cycle, x));

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
            .collect::<String>()
            .chars()
            .collect::<Vec<_>>()
            .chunks(40)
            .intersperse(&['\n'])
            .flatten()
            .collect::<String>()
    );
}

fn main() {
    println!(
        "sum of signal strength at given cycles: {}",
        signal_strength(include_str!("input.txt"))
    );
    draw_crt(include_str!("input.txt"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(signal_strength(include_str!("test.txt")), 13140);
    }
}
