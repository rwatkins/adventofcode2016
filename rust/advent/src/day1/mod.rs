mod path_tracker;

use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

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

fn file_contents(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Converts a string like "L1, R2, R5" into a collection of Steps

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
    let s = file_contents("input.txt").expect("Error getting file contents");
    let steps = steps_from_str(&s);
    let mut path_tracker = PathTracker::new();
    let dist = distance(&steps, &mut path_tracker);
    println!("Distance to Easter Bunny HQ: {}", dist);

    let dist2 = path_tracker.distance_to_first_point_visited_twice();
    println!("Distance to first point visited twice: {:?}", dist2);
}
