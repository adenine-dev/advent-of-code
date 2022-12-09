// adapted from std::iterator::TakeWhile

#[derive(Clone)]
pub struct TakeWhileInclusive<I: Iterator, P: FnMut(&I::Item) -> bool> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: Iterator, P: FnMut(&I::Item) -> bool> TakeWhileInclusive<I, P> {
    pub fn new(iter: I, predicate: P) -> Self {
        TakeWhileInclusive {
            iter,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for TakeWhileInclusive<I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            let next = self.iter.next()?;
            if !(self.predicate)(&next) {
                self.flag = true;
            }
            Some(next)
        }
    }
}

fn visible_trees(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            let up_iter = grid.iter().map(|row| row[j]).rev().take(grid.len() - i - 1);
            let down_iter = grid.iter().map(|row| row[j]).take(i);
            let left_iter = row.iter().rev().take(row.len() - j - 1);
            let right_iter = row.iter().take(j);

            if *right_iter.max().unwrap_or(&-1) < height
                || *left_iter.max().unwrap_or(&-1) < height
                || down_iter.max().unwrap_or(-1) < height
                || up_iter.max().unwrap_or(-1) < height
            {
                sum += 1;
            }
        }
    }

    sum
}

fn best_tree_score(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut scores =
        vec![vec![(usize::MAX, usize::MAX, usize::MAX, usize::MAX); grid.len()]; grid.len()];

    let x = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &height)| {
                    let down = grid.iter().map(|x| x[j]).skip(i + 1);
                    let up = grid.iter().map(|x| x[j]).rev().skip(grid.len() - i);
                    let right = row.iter().skip(j + 1);
                    let left = row.iter().rev().skip(row.len() - j);

                    scores[i][j] = (
                        TakeWhileInclusive::new(down, |x| *x < height).count(),
                        TakeWhileInclusive::new(up, |x| *x < height).count(),
                        TakeWhileInclusive::new(right, |x| **x < height).count(),
                        TakeWhileInclusive::new(left, |x| **x < height).count(),
                    );
                    scores[i][j].0 * scores[i][j].1 * scores[i][j].2 * scores[i][j].3
                })
                .max()
        })
        .max()
        .unwrap() as u32;
    x
}

fn main() {
    println!(
        "number of visible trees: {}",
        visible_trees(include_str!("input.txt"))
    );

    println!(
        "best tree score: {}",
        best_tree_score(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(visible_trees(include_str!("test.txt")), 21);
        assert_eq!(best_tree_score(include_str!("test.txt")), 8);
    }
}
