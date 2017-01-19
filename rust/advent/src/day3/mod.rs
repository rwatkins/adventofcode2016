use std::collections::VecDeque;
use std::str::{FromStr, Lines};
use super::util;

struct Triangle { a: usize, b: usize, c: usize }

impl Triangle {
    pub fn try_new(a: usize, b: usize, c: usize) -> Result<Self, ()> {
        let t = Triangle { a: a, b: b, c: c };
        if t.is_valid() {
            Ok(t)
        } else {
            Err(())
        }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c &&
            self.a + self.c > self.b &&
            self.b + self.c > self.a
    }
}

impl FromStr for Triangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim()
            .split_whitespace()
            .map(|part| usize::from_str(part).expect("Could not parse number"))
            .collect::<Vec<_>>();
        let t = Triangle {
            a: parts[0],
            b: parts[1],
            c: parts[2],
        };
        if t.a + t.b > t.c && t.a + t.c > t.b && t.b + t.c > t.a {
            Ok(t)
        } else {
            Err(())
        }
    }
}

struct Input<'a> {
    lines: Vec<(String, String, String)>,
    next_queue: VecDeque<(usize, usize, usize)>,
    cursor: 0,
}

impl<'a> Input<'a> {
    pub fn new(s: &str) -> Input {
        Input {
            lines: s.lines()
                .map(|l| l.to_string())
                .chunks(3)
                .collect::<Vec<_>>(),
            next_queue: VecDeque::new(),
        }
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.next_queue.pop_front() {
            return Some(item);
        }

        let lines = self.lines.take(3)
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|i| usize::from_str(i).expect("Could not parse number"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let item1 = (lines[0][0], lines[1][0], lines[2][0]);
        let item2 = (lines[0][1], lines[1][1], lines[2][1]);
        let item3 = (lines[0][2], lines[1][2], lines[2][2]);

        self.next_queue.push_back(item2);
        self.next_queue.push_back(item3);

        Some(item1)
    }
}

fn count_triangles() -> usize {
    let input = util::read_file("day3_input.txt")
        .expect("Could not read day3_input.txt");
    input.lines()
        .map(|line| Triangle::from_str(&line))
        .filter(|t| Result::is_ok(&t))
        .count()
}

pub fn main() {
    println!("DAY 3");
    println!("Possible triangles: {}", count_triangles());
}
