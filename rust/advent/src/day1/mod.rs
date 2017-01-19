mod path_tracker;

use std::str::FromStr;

use super::util;
use self::path_tracker::PathTracker;

enum Axis {
    X,
    Y,
}

enum Direction {
    Pos,
    Neg,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Step(Turn, i32);


// Iterates over steps and traces the path taken along a grid to reach
// the final destination.
//
// A step represents in which direction we should turn from the current
// direction we're facing, and how many blocks we should walk in that
// direction.
//
// For example, when starting from (0, 0) and facing in the positive
// direction along the Y axis, a sequence of steps like [R4, R5, L8]
// leaves us at (-4, -5) and facing in the negative direction along the
// X axis.

fn distance(steps: &[Step], path_tracker: &mut PathTracker) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut cur_axis = Axis::Y;
    let mut cur_direction = Direction::Pos;

    for step in steps {
        let (old_x, old_y) = (x, y);

        match (cur_axis, cur_direction, step) {
            (Axis::X, Direction::Pos, &Step(Turn::Left, i)) |
            (Axis::X, Direction::Neg, &Step(Turn::Right, i)) => {
                cur_axis = Axis::Y;
                cur_direction = Direction::Pos;
                y += i;
            }
            (Axis::X, Direction::Pos, &Step(Turn::Right, i)) |
            (Axis::X, Direction::Neg, &Step(Turn::Left, i)) => {
                cur_axis = Axis::Y;
                cur_direction = Direction::Neg;
                y -= i;
            }
            (Axis::Y, Direction::Pos, &Step(Turn::Left, i)) |
            (Axis::Y, Direction::Neg, &Step(Turn::Right, i)) => {
                cur_axis = Axis::X;
                cur_direction = Direction::Neg;
                x -= i;
            }
            (Axis::Y, Direction::Pos, &Step(Turn::Right, i)) |
            (Axis::Y, Direction::Neg, &Step(Turn::Left, i)) => {
                cur_axis = Axis::X;
                cur_direction = Direction::Pos;
                x += i;
            }
        }

        path_tracker.add_path((old_x, old_y), (x, y));
    }

    x.abs() + y.abs()
}


// Converts a string like "L1, R2, R5" into a collection of Steps.

fn steps_from_str(s: &str) -> Vec<Step> {
    s.trim().split(", ")
        .map(|s| s.split_at(1))
        .map(|(s, i_s)| (s, i32::from_str(i_s).expect("Could not parse string to i32")))
        .map(|(s, i)| match s {
            "L" => Step(Turn::Left, i),
            "R" => Step(Turn::Right, i),
            _ => unreachable!(),
        })
        .collect::<Vec<Step>>()
}


pub fn main() {
    println!("DAY 1");
    let s = util::read_file("day1_input.txt").expect("Error getting file contents");
    let steps = steps_from_str(&s);
    let mut path_tracker = PathTracker::new();
    let dist = distance(&steps, &mut path_tracker);
    println!("Distance to Easter Bunny HQ: {}", dist);

    let dist2 = path_tracker.distance_to_first_point_visited_twice();
    println!("Distance to first point visited twice: {:?}", dist2);
}
