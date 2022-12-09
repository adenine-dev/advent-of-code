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

fn visited_with_commands(input: &str) -> u32 {
    let mut head = Point2::default();
    let mut tail = Point2::default();

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
            head += dir;
            if tail.should_move(&head) {
                let dist = head - tail;
                tail += Point2 {
                    x: dist.x.signum(),
                    y: dist.y.signum(),
                };
            }
            tail
        })
        .collect::<HashSet<_>>()
        .len() as u32
}

fn main() {
    println!(
        "number of positions visited: {}",
        visited_with_commands(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(visited_with_commands(include_str!("test.txt")), 13);
    }
}
