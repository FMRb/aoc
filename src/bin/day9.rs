use anyhow::Result;
use nom::{
    character::complete::{alpha1, digit0, newline, space0},
    multi::separated_list1,
    IResult,
};
use std::{collections::HashSet, hash::Hash};

enum Motion {
    Up(u32),
    Down(u32),
    Right(u32),
    Left(u32),
}
fn parse_motion(input: &str) -> IResult<&str, Motion> {
    let (input, dir) = alpha1(input)?;
    let (input, _) = space0(input)?;
    let (input, value) = digit0(input)?;
    let value = value.parse::<u32>().unwrap();

    Ok((
        input,
        match dir {
            "U" => Motion::Up(value),
            "D" => Motion::Down(value),
            "R" => Motion::Right(value),
            "L" => Motion::Left(value),
            _ => panic!("Error in motion"),
        },
    ))
}
fn parse_motions(input: &str) -> IResult<&str, Vec<Motion>> {
    Ok(separated_list1(newline, parse_motion)(input)?)
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new() -> Self {
        Self { knots: Vec::new() }
    }
}

#[derive(Debug)]
struct Knot {
    pos: (isize, isize),
    visited_pos: Vec<(isize, isize)>,
}

impl Knot {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            visited_pos: vec![(0,0)],
        }
    }

    fn distance(&self, other_knot: &Knot) -> (isize, isize) {
        let (x, y) = other_knot.pos;
        (x - self.pos.0, y - self.pos.1)
    }

    fn update_position(&mut self, x: isize, y: isize) {
        if x == 0 && y == 0 {
            return;
        }
        self.pos = (self.pos.0 + x, self.pos.1 + y);
        self.visited_pos.push(self.pos);
    }
}

fn get_dir(motion: Motion) -> (u32, (isize, isize)) {
    match motion {
        Motion::Up(steps) => (steps, (0, 1)),
        Motion::Down(steps) => (steps, (0, -1)), 
        Motion::Right(steps) => (steps, (1, 0)),
        Motion::Left(steps) => (steps, (-1, 0)),
    }
}
fn increase_direction(diff: (isize, isize)) -> (isize, isize) {
    let (x, y) = diff;
    (
        if x != 0 {
            x / isize::abs(x)
        } else {
            0
        },
        if y != 0 {
            y / isize::abs(y)
        } else {
            0
        }
    )
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/9.input")?;
    let (_, motions) = parse_motions(&input).unwrap();

    let mut rope = Rope::new();

    for _ in 0..10 {
        let knot = Knot::new();
        rope.knots.push(knot);
    }

    for motion in motions {
        let (steps, dir) = get_dir(motion);
        for _ in 0..steps {
            let (head_pos_x, head_pos_y) = rope.knots[0].pos;
            rope.knots[0].update_position(dir.0, dir.1);
            let mut prev_knot = &rope.knots[0];
            for i in 1..rope.knots.len() {
                let diff = rope.knots[i].distance(prev_knot);
                if isize::abs(diff.0) >= 2 || isize::abs(diff.1) >= 2 {
                    let incr = increase_direction(diff);
                    rope.knots[i].update_position(incr.0, incr.1);
                }
                prev_knot = &rope.knots[i];
            }
        }
    }
    let visited_pos: HashSet<&(isize, isize)> = HashSet::from_iter(rope
        .knots
        .last()
        .unwrap()
        .visited_pos
        .iter()
        .clone());

    println!("Part 1: {}", visited_pos.len());
    Ok(())
}
