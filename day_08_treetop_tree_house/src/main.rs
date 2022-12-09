fn visible_trees(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut bool_grid = vec![vec![0; grid.len()]; grid.len()];
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
                bool_grid[i][j] = 1;
            }
        }
    }

    // dbg!(bool_grid);

    sum
}

fn main() {
    println!(
        "number of visible trees: {}",
        visible_trees(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(visible_trees(include_str!("test.txt")), 21);
    }
}
