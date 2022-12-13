#![feature(let_chains)]

use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    steps: u32,
    idx: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn climb_hill(input: &str) -> u32 {
    const INVALID: usize = usize::MAX;

    let width = input.lines().next().unwrap().len();
    // let height = input.lines().count();

    let mut start = INVALID;
    let mut end = INVALID;
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    b'S' => {
                        start = x + width * y;
                        0
                    }
                    b'E' => {
                        end = x + width * y;
                        25
                    }
                    _ => (c - b'a') as i8,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut steps = vec![u32::MAX; map.len()];
    let mut visited = vec![];
    let mut unvisited = BinaryHeap::from(vec![Node {
        idx: start,
        steps: 0,
    }]);

    while let Some(node) = unvisited.pop() {
        if node.idx == end {
            return node.steps;
        }

        if node.steps > steps[node.idx] {
            continue;
        }

        for d in [1, -1, width as isize, -(width as isize)] {
            if let Some(nidx) = (node.idx as isize).checked_add(d).map(|x| x as usize)
                && nidx < map.len()
                && !visited.contains(&nidx)
                && node.steps + 1 < steps[nidx]
                && map[nidx] - map[node.idx] <= 1 {
                // for some reason there's not a binary heap contains fn :/ so this isn't super efficient
                // not like it is anyway
                // but still
                
                unvisited.push(Node {
                    steps: node.steps + 1,
                    idx: nidx,
                });
                steps[nidx] = node.steps + 1;
            }
        }
        visited.push(node.idx);
    }

    steps[end]
}

fn main() {
    println!(
        "minimum steps to climb hill: {}",
        climb_hill(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(climb_hill(include_str!("test.txt")), 31);
    }
}
