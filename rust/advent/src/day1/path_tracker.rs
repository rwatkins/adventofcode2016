use std::collections::HashSet;

pub struct PathTracker {
    visited: HashSet<(i32, i32)>,
    visited_multi: Vec<(i32, i32)>,
}

impl PathTracker {
    pub fn new() -> PathTracker {
        PathTracker {
            visited: HashSet::new(),
            visited_multi: vec![],
        }
    }

    pub fn add_path(&mut self, a: (i32, i32), b: (i32, i32)) {
        let (a_x, a_y) = a;
        let (b_x, b_y) = b;

        if a_x == b_x {
            // X axis doesn't change, so we're adding points along the Y axis
            if b_y >= a_y {
                // Positive direction along the axis
                for i in 0..(b_y - a_y) {
                    self.add_point(a_x, a_y + i);
                }
            } else {
                // Negative direction along the axis
                for i in 0..(a_y - b_y) {
                    self.add_point(a_x, a_y - i);
                }
            }
        } else if a_y == b_y {
            // Y axis doesn't change, so we're adding points along the X axis
            if b_x >= a_x {
                // Positive direction along the axis
                for i in 0..(b_x - a_x) {
                    self.add_point(a_x + i, a_y);
                }
            } else {
                // Negative direction along the axis
                for i in 0..(a_x - b_x) {
                    self.add_point(a_x - i, a_y);
                }
            }
        } else {
            // Path between a and b should be straight along x or y
            // axis. Diagonal path is unexpected.
            unreachable!();
        }
    }

    pub fn distance_to_first_point_visited_twice(&self) -> Option<i32> {
        self.first_point_visited_twice().map(|&(x, y)| x.abs() + y.abs())
    }

    fn first_point_visited_twice(&self) -> Option<&(i32, i32)> {
        self.visited_multi.first()
    }

    fn add_point(&mut self, x: i32, y: i32) {
        let point = (x, y);
        if self.visited.contains(&point) {
            self.visited_multi.push(point);
        } else {
            self.visited.insert(point);
        }
    }
}
