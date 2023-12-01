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

fn build_map(input: &str) -> (Vec<i8>, usize, usize, usize) {
    const INVALID: usize = usize::MAX;

    let width = input.lines().next().unwrap().len();

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

    (map, start, end, width)
}

fn dijkstra_go_spin(map: &[i8], start: usize, end: usize, width: usize) -> u32 {
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

fn climb_hill(input: &str) -> u32 {
    let (map, start, end, width) = build_map(input);
    dijkstra_go_spin(&map, start, end, width)
}

fn hill_climb(input: &str) -> u32 {
    let (map, _, end, width) = build_map(input);
    map.iter()
        .enumerate()
        .filter_map(|(idx, height)| {
            if *height == 0 {
                Some(dijkstra_go_spin(&map, idx, end, width))
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn main() {
    println!(
        "minimum steps to climb hill: {}",
        climb_hill(include_str!("input.txt"))
    );
    println!(
        "minimum steps to hill climb: {}",
        hill_climb(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(climb_hill(include_str!("test.txt")), 31);
        assert_eq!(hill_climb(include_str!("test.txt")), 29);
    }
}
