use std::vec;

fn drop_sand(input: &str) -> u32 {
    let mut width = 0;
    let mut height = 0;
    let mut min_x = usize::MAX;

    let mut coords = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|coord| {
                    let (x, y) = coord.split_once(',').unwrap();
                    let x = x.trim().parse().unwrap();
                    let y = y.trim().parse().unwrap();
                    width = width.max(x);
                    height = height.max(y);
                    min_x = min_x.min(x);
                    (x, y)
                })
                // .(|(x, y)| (x - min_x, y - min_y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    min_x -= 1;
    width -= min_x;
    coords.iter_mut().for_each(|v| {
        v.iter_mut().for_each(|(x, _)| {
            *x -= min_x;
        })
    });

    let mut cave = vec![vec![false; width + 1]; height + 1];

    coords.iter().for_each(|line| {
        line.windows(2).for_each(|x| {
            let &[(x1, y1), (x2, y2)] = x else { panic!("oof") };
            for x in 0..=x1.abs_diff(x2) {
                for y in 0..=y1.abs_diff(y2) {
                    cave[y1.min(y2) + y][x1.min(x2) + x] = true;
                }
            }
        })
    });

    let mut grains = 0;
    loop {
        let mut x = 500 - min_x;
        let mut y = 0;
        loop {
            if height < y + 1 || width < x {
                return grains;
            }
            if !cave[y + 1][x] {
                y += 1;
            } else if !cave[y + 1][x - 1] {
                y += 1;
                x -= 1;
            } else if !cave[y + 1][x + 1] {
                y += 1;
                x += 1;
            } else {
                cave[y][x] = true;
                break;
            }
        }
        grains += 1;
    }
}

fn drop_sand1(input: &str) -> u32 {
    let mut width = 0;
    let mut height = 0;
    let mut min_x = usize::MAX;

    let mut coords = input
        .lines()
        .map(|line| {
            line.split("->")
                .map(|coord| {
                    let (x, y) = coord.split_once(',').unwrap();
                    let x = x.trim().parse().unwrap();
                    let y = y.trim().parse().unwrap();
                    width = width.max(x);
                    height = height.max(y);
                    min_x = min_x.min(x);
                    (x, y)
                })
                // .(|(x, y)| (x - min_x, y - min_y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    min_x -= 1;
    width -= min_x;
    height += 2;
    let base = ((2 * height) + 1) + 2;
    min_x -= (base - width) / 2;
    width = base;
    coords.iter_mut().for_each(|v| {
        v.iter_mut().for_each(|(x, _)| {
            *x -= min_x;
        })
    });
    coords.push(vec![(0, height), (width, height)]);

    let mut cave = vec![vec![false; width + 1]; height + 1];

    coords.iter().for_each(|line| {
        line.windows(2).for_each(|x| {
            let &[(x1, y1), (x2, y2)] = x else { panic!("oof") };
            for x in 0..=x1.abs_diff(x2) {
                for y in 0..=y1.abs_diff(y2) {
                    cave[y1.min(y2) + y][x1.min(x2) + x] = true;
                }
            }
        })
    });

    let mut grains = 0;
    loop {
        let mut x = 500 - min_x;
        let mut y = 0;
        loop {
            if !cave[y + 1][x] {
                y += 1;
            } else if !cave[y + 1][x - 1] {
                y += 1;
                x -= 1;
            } else if !cave[y + 1][x + 1] {
                y += 1;
                x += 1;
            } else {
                cave[y][x] = true;
                break;
            }
        }
        grains += 1;
        if x == 500 - min_x && y == 0 {
            return grains;
        }
    }
}

fn main() {
    println!(
        "units of sand that will drop into the void: {}",
        drop_sand(include_str!("input.txt"))
    );
    println!(
        "units of sand that will drop onto you: {}",
        drop_sand1(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(drop_sand(include_str!("test.txt")), 24);
        assert_eq!(drop_sand1(include_str!("test.txt")), 93);
    }
}
