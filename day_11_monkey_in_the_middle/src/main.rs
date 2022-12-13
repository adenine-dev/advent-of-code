use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Add(u32),
    Mul(u32),
    Sqr,
}

impl Operation {
    fn upon(&self, worry: u32) -> u32 {
        match self {
            Self::Add(x) => worry + x,
            Self::Mul(x) => worry * x,
            Self::Sqr => worry * worry,
        }
    }
}

#[derive(Debug)]
struct Test {
    pub divisible_by: u32,
    pub if_true: usize,
    pub if_false: usize,
}

impl Test {
    fn throws_to(&self, worry: u32) -> usize {
        if worry % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    pub idx: usize,
    pub operation: Operation,
    pub test: Test,
    pub items: VecDeque<u32>,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = input
        .split("\n\n")
        .map(|input| {
            let mut lines = input.lines();
            let idx = {
                let (_, idx) = lines.next().unwrap().split_once(' ').unwrap();
                idx.split_once(':').unwrap().0.parse::<usize>().unwrap()
            };
            let items = lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect();
            let operation = {
                let (_, expr) = lines.next().unwrap().split_once("= ").unwrap();
                let (lhs, rhs) = expr.split_once(['+', '*']).unwrap();
                let lhs = lhs.trim();
                let rhs = rhs.trim();
                let lhs_n = lhs.parse::<u32>().unwrap_or(0);
                let rhs_n = rhs.parse::<u32>().unwrap_or(0);
                match if expr.find('*').is_some() { '*' } else { '+' } {
                    '*' => {
                        if lhs_n == 0 && rhs_n == 0 && lhs == "old" && rhs == "old" {
                            Operation::Sqr
                        } else if lhs_n == 0 {
                            Operation::Mul(rhs_n)
                        } else if rhs_n == 0 {
                            Operation::Mul(lhs_n)
                        } else {
                            panic!("oop");
                        }
                    }
                    '+' => {
                        if lhs_n == 0 && rhs_n == 0 && lhs == "old" && rhs == "old" {
                            Operation::Mul(2)
                        } else if lhs_n == 0 {
                            Operation::Add(rhs_n)
                        } else if rhs_n == 0 {
                            Operation::Add(lhs_n)
                        } else {
                            panic!("oop");
                        }
                    }
                    _ => panic!("oop"),
                }
            };
            let test = Test {
                divisible_by: lines
                    .next()
                    .unwrap()
                    .split_once("Test: divisible by ")
                    .unwrap()
                    .1
                    .parse::<u32>()
                    .unwrap(),
                if_true: lines
                    .next()
                    .unwrap()
                    .split_once("If true: throw to monkey ")
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
                if_false: lines
                    .next()
                    .unwrap()
                    .split_once("If false: throw to monkey ")
                    .unwrap()
                    .1
                    .parse::<usize>()
                    .unwrap(),
            };
            Monkey {
                idx,
                items,
                operation,
                test,
            }
        })
        .collect::<Vec<_>>();

    monkeys.sort_unstable_by_key(|m| m.idx);
    monkeys
}

fn monkeys_doing_monkey_things(input: &str) -> u32 {
    let mut monkeys = parse_monkeys(input);
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop_front().unwrap();
                inspections[i] += 1;
                let item = monkeys[i].operation.upon(item) / 3;
                let target = monkeys[i].test.throws_to(item);
                monkeys[target].items.push_back(item);
            }
        }
    }
    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn main() {
    println!(
        "level of monkey business after 20 rounds of stuff-slinging simian shenanigans: {}",
        monkeys_doing_monkey_things(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(monkeys_doing_monkey_things(include_str!("test.txt")), 10605);
    }
}
