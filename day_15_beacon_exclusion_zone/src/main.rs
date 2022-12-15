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

        let dist = sx.abs_diff(bx) + sy.abs_diff(by);
        let row_intersection_count = (2 * (dist + 1 - sy.abs_diff(row)).max(0)) as i32;

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

fn main() {
    println!(
        "number of positions not panicking: {}",
        excluded_positions(include_str!("input.txt"), 2000000)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(excluded_positions(include_str!("test.txt"), 10), 26);
    }
}
