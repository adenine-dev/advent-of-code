use core::ops::*;
use std::collections::HashSet;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn should_move(&self, other: &Point2) -> bool {
        let mdist = (*self - *other).abs();
        !(mdist.x == 1 && mdist.y == 1 || mdist.x + mdist.y == 1)
    }
}

impl Add<Point2> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Point2) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign<Point2> for Point2 {
    fn add_assign(&mut self, rhs: Point2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point2> for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Point2) -> Self::Output {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign<Point2> for Point2 {
    fn sub_assign(&mut self, rhs: Point2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

fn visited_with_commands(input: &str, tail_len: usize) -> u32 {
    let mut knots = vec![Point2::default(); 1 + tail_len];

    input
        .lines()
        .flat_map(|line| {
            let (dir, amt) = line.split_once(' ').unwrap();
            let dir = match dir {
                "U" => Point2 { x: 0, y: 1 },
                "D" => Point2 { x: 0, y: -1 },
                "L" => Point2 { x: -1, y: 0 },
                "R" => Point2 { x: 1, y: 0 },
                _ => panic!("bad input :<"),
            };
            vec![dir; amt.parse::<usize>().unwrap()]
        })
        .map(|dir| {
            knots[0] += dir;
            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = &mut knots[i];
                if tail.should_move(&head) {
                    let dist = head - *tail;
                    *tail += Point2 {
                        x: dist.x.signum(),
                        y: dist.y.signum(),
                    };
                }
            }

            *knots.last().unwrap()
        })
        .collect::<HashSet<_>>()
        .len() as u32
}

fn main() {
    println!(
        "number of positions visited (tail len 1): {}",
        visited_with_commands(include_str!("input.txt"), 1)
    );
    println!(
        "number of positions visited (tail len 9): {}",
        visited_with_commands(include_str!("input.txt"), 9)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(visited_with_commands(include_str!("test.txt"), 1), 13);
        assert_eq!(visited_with_commands(include_str!("test.txt"), 9), 1);
    }
}
