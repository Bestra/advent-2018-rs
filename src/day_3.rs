use std::i32;
pub fn part_1(input: u32) -> String {
    format!("{}", Spiral::new().dist(input as usize))
}

pub fn part_2(input: u32) -> String {
    "Foo".to_string()
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
    Right
}

struct Spiral {
    ring: i32,
    x: i32,
    y: i32,
    dir: Dir
}

impl Spiral {
    fn new() -> Spiral {
        Spiral {ring: 0, x: 0, y: 0, dir: Dir::Right}
    }

    fn needs_next_ring(&self) -> bool {
        self.x == self.ring && self.y == -self.ring
    }

    fn needs_corner(&self) -> bool {
        self.x.abs() == self.ring && self.y.abs() == self.ring
    }

    fn dist(&mut self, address: usize) -> i32 {
        let (x, y) = self.nth(address - 2).unwrap();
        x.abs() + y.abs()
    }

    fn next_dir(&mut self) {
        let new = match self.dir {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up
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
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_next_ring() {
            self.next_ring();
            Some((self.x, self.y))
        }  else {
            if self.needs_corner() {
                self.next_dir();
            }
            self.next_pos();
            Some((self.x, self.y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_1() {
        let mut s = Spiral::new();
        assert_eq!(s.next(), Some((1, 0)));
        assert_eq!(s.next(), Some((1, 1)));
        assert_eq!(s.next(), Some((0, 1)));
        assert_eq!(s.next(), Some((-1, 1)));
    }

    #[test]
    fn check_17() {
        let mut s = Spiral::new();
        assert_eq!(s.nth(15), Some((-2, 2)));
    }

    #[test]
    fn check_dist() {
        assert_eq!(Spiral::new().dist(12), 3);
        assert_eq!(Spiral::new().dist(23), 2);
        assert_eq!(Spiral::new().dist(1024), 31);
    }
}
