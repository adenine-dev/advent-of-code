use std::collections::HashSet;

fn excluded_positions(input: &str, row: i32) -> u32 {
    let mut beacons = HashSet::new();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    input.lines().for_each(|line| {
        let [sx, sy, bx, by] = line
                .split(['=', ',', ':'])
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>()[..] else { panic!("oof") };

        if by == row {
            beacons.insert(bx);
        }

        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as i32;
        let row_intersection_count = (2 * (dist + 1 - (sy.abs_diff(row)).max(0) as i32)) as i32;

        if row_intersection_count == 0 {
            return;
        }

        let row_intersection_count = row_intersection_count - 1;
        let rx = sx - (row_intersection_count) / 2;
        min = min.min(rx);
        max = max.max(rx + row_intersection_count);
    });

    ((max - min) - beacons.len() as i32) as u32
}

fn tuning_freq(input: &str, min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> u64 {
    let coords = input
        .lines()
        .map(|line| {
            let [sx, sy, bx, by] = line
                .split(['=', ',', ':'])
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<_>>()[..] else { panic!("oof") };
            let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as i64;
            [sx, sy, dist]
        })
        .collect::<Vec<_>>();

    for row in min_y..=max_y {
        let mut ranges = coords
            .iter()
            .flat_map(|&[sx, sy, dist]| {
                let row_intersection_count =
                    (2 * (dist + 1 - (sy.abs_diff(row)).max(0) as i64)) as i64;

                if row_intersection_count <= 0 {
                    return None;
                }

                let row_intersection_count = row_intersection_count - 1;
                let rx = sx - (row_intersection_count) / 2;
                Some((rx.max(min_x), (rx + row_intersection_count - 1).min(max_x)))
            })
            .collect::<Vec<_>>();
        ranges.sort_by(|&a, &b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        loop {
            let prevlen = ranges.len();

            ranges = ranges
                .chunks(2)
                .flat_map(|x| {
                    if x.len() == 1 || (x[0].0 <= x[1].0 && x[1].1 <= x[0].1) {
                        [Some(x[0]), None]
                    } else if x[1].0 <= x[0].1 || x[1].0 + 1 == x[0].1 {
                        [Some((x[0].0, x[1].1)), None]
                    } else {
                        [Some(x[0]), Some(x[1])]
                    }
                })
                .flatten()
                .collect::<Vec<_>>();

            if prevlen == ranges.len() {
                break;
            }
        }

        if ranges.len() != 1 {
            return ((ranges[0].1 + 1) * 4000000 + row) as u64;
        }
    }

    0
}

fn main() {
    println!(
        "number of positions not panicking: {}",
        excluded_positions(include_str!("input.txt"), 2000000)
    );
    println!(
        "tuning frequency of distress signal: {}",
        tuning_freq(include_str!("input.txt"), 0, 0, 4000000, 4000000)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(excluded_positions(include_str!("test.txt"), 10), 26);
        assert_eq!(
            tuning_freq(include_str!("test.txt"), 0, 0, 20, 20),
            56000011
        );
    }
}
