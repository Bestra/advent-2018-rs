use std::i32;
use std::collections::HashMap;

pub fn part_1(input: u32) -> String {
    // TODO: Part 1 will overflow after being modified for part 2
    format!("{}", Spiral::new().dist(input as usize))
}

pub fn part_2(input: i32) -> String {
    let (_, _, v) = Spiral::new().find(|&(_, _, x)| x > input).unwrap();
    format!("{}", v)
}

// "17  16  15  14  13
// 18   5   4   3  12
// 19   6   1   2  11
// 20   7   8   9  10
// 21  22  23---> ..."

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Spiral {
    ring: i32,
    x: i32,
    y: i32,
    dir: Dir,
    entries: HashMap<(i32, i32), i32>,
}

impl Spiral {
    fn new() -> Spiral {
        let mut h = HashMap::new();
        h.insert((0, 0), 1);
        Spiral {
            ring: 0,
            x: 0,
            y: 0,
            dir: Dir::Right,
            entries: h,
        }
    }

    fn needs_next_ring(&self) -> bool {
        self.x == self.ring && self.y == -self.ring
    }

    fn neighbor_sum(&self) -> i32 {
        let offsets = vec![
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        let mut sum = 0;
        for &(x, y) in &offsets {
            if let Some(n) = self.entries.get(&(self.x + x, self.y + y)) {
                sum += n;
            }
        }
        sum
    }

    fn needs_corner(&self) -> bool {
        self.x.abs() == self.ring && self.y.abs() == self.ring
    }

    fn dist(&mut self, address: usize) -> i32 {
        let (x, y, _) = self.nth(address - 2).unwrap();
        x.abs() + y.abs()
    }

    fn next_dir(&mut self) {
        let new = match self.dir {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        };
        self.dir = new;
    }

    fn next_pos(&mut self) {
        let (x, y) = match self.dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        self.x += x;
        self.y += y;
    }

    fn next_ring(&mut self) {
        self.ring += 1;
        self.dir = Dir::Up;
        self.x += 1;
    }
}

impl Iterator for Spiral {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_next_ring() {
            self.next_ring();
        } else {
            if self.needs_corner() {
                self.next_dir();
            }
            self.next_pos();
        }
        let s = self.neighbor_sum();
        self.entries.insert((self.x, self.y), s);
        Some((self.x, self.y, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_1() {
        let mut s = Spiral::new();
        assert_eq!(s.next(), Some((1, 0, 1)));
        assert_eq!(s.next(), Some((1, 1, 2)));
        assert_eq!(s.next(), Some((0, 1, 4)));
        assert_eq!(s.next(), Some((-1, 1, 5)));
    }

    #[test]
    fn check_17() {
        let mut s = Spiral::new();
        assert_eq!(s.nth(15), Some((-2, 2, 147)));
    }

    #[test]
    fn check_dist() {
        assert_eq!(Spiral::new().dist(12), 3);
        assert_eq!(Spiral::new().dist(23), 2);
        // assert_eq!(Spiral::new().dist(1024), 31);
    }
}
