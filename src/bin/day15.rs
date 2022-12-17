use std::{collections::HashMap};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Sensor {
    pos: (i64, i64),
    closest_beacon: (i64, i64),
    manhattan_dist: i64,
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = complete::i64(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = complete::i64(input)?;

    Ok((
        input,
        Sensor {
            pos: (sensor_x, sensor_y),
            closest_beacon: (beacon_x, beacon_y),
            manhattan_dist: Tunnels::calculate_manhattan_distance(
                (sensor_x, sensor_y),
                (beacon_x, beacon_y),
            ),
        },
    ))
}

fn parse_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, parse_sensor)(input)
}

struct Tunnels {
    grid: HashMap<(i64, i64), Tile>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Sensor,
    Beacon,
    Scanned,
}

impl Tunnels {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    fn calculate_manhattan_distance(point_a: (i64, i64), point_b: (i64, i64)) -> i64 {
        i64::abs(point_a.0 - point_b.0) + i64::abs(point_a.1 - point_b.1)
    }

    fn draw_sensor_reach(&mut self, sensor: &Sensor) {
        let sensor_beacon_dist =
            Tunnels::calculate_manhattan_distance(sensor.pos, sensor.closest_beacon);

        self.grid.insert(sensor.pos, Tile::Sensor);
        self.grid.insert(sensor.closest_beacon, Tile::Beacon);
        for j in sensor.pos.1 - sensor_beacon_dist..=sensor.pos.1 + sensor_beacon_dist {
            for i in sensor.pos.0 - sensor_beacon_dist..=sensor.pos.0 + sensor_beacon_dist {
                if Tunnels::calculate_manhattan_distance(sensor.pos, (i, j)) <= sensor_beacon_dist
                    && !self.grid.contains_key(&(i, j))
                {
                    self.grid.insert((i, j), Tile::Scanned);
                }
            }
        }
    }

    fn display(&self) {
        for j in -2..22 {
            for i in -2..25 {
                print!(
                    "{}",
                    if self.grid.get(&(i, j)).is_some() {
                        '#'
                    } else {
                        '.'
                    }
                )
            }
            println!()
        }
    }
}

fn exercise_one(sensors: &Vec<Sensor>, min_pos: (i64, i64), max_pos: (i64, i64)) -> i64 {
    let target_row = 2_000_000;
    let mut count = 0;

    for i in min_pos.0 - target_row..max_pos.0 + target_row {
        let tile_pos = (i, target_row);
        let mut sensor_reaches = false;
        let mut is_pos_beacon = false;
        for sensor in sensors.iter() {
            let dist = Tunnels::calculate_manhattan_distance(sensor.pos, tile_pos);
            if dist <= sensor.manhattan_dist {
                sensor_reaches = true;
            }

            if sensor.pos.0 == tile_pos.0 && sensor.pos.1 == tile_pos.1
                || sensor.closest_beacon.0 == tile_pos.0 && sensor.closest_beacon.1 == tile_pos.1
            {
                is_pos_beacon = true;
            }
        }
        if sensor_reaches && !is_pos_beacon {
            count += 1;
        }
    }
    count
}

fn exercise_two(
    sensors: &Vec<Sensor>,
    max_cap: i64
) -> i64 {

    let mut grid = vec![vec![0..=max_cap]; max_cap as usize + 1];
    for sensor in sensors {
        let floor = 0.max(sensor.pos.1 - sensor.manhattan_dist);
        let ceiling = max_cap.min(sensor.pos.1 + sensor.manhattan_dist);
        for j in floor..=ceiling {
            let dist = (sensor.pos.1 - j).abs();

            let overlap = sensor.manhattan_dist - dist;
            let left = 0.max(sensor.pos.0 - overlap);
            let right = max_cap.min(sensor.pos.0 + overlap);

            let mut new_lookup = Vec::new();
            // start................end
            //       left.....right
            //.......right
            //                  left.....right
            for row in &grid[j as usize] {
                let start = *row.start();

                if start > right {
                    new_lookup.push(row.clone());
                    continue;
                }
                let end = *row.end();
                if end < left {
                    new_lookup.push(row.clone());
                    continue;
                }
                if start < left {
                    new_lookup.push(start..=left-1);
                }

                if end > right {
                    new_lookup.push(right+1..=end);
                }
            }
            grid[j as usize] = new_lookup;
        }
    }

    for (j, row) in grid.iter().enumerate() {
        if !row.is_empty() {
            let i = *row[0].start();
            return i * 4_000_000 + j as i64;
        }
    }

    -1
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/15.input")?;
    let (_, sensors) = parse_sensors(&input).unwrap();

    let mut tunnels = Tunnels::new();

    // for sensor in sensors.iter() {
    //     tunnels.draw_sensor_reach(sensor);
    // }

    let mut min_pos = (i64::MAX, i64::MAX);
    let mut max_pos = (0, 0);
    for sensor in sensors.iter() {
        let min_point = (
            sensor.pos.0 - sensor.manhattan_dist,
            sensor.pos.1 - sensor.manhattan_dist,
        );
        let max_point = (
            sensor.pos.0 + sensor.manhattan_dist,
            sensor.pos.1 + sensor.manhattan_dist,
        );

        if min_point.0 < min_pos.0 {
            min_pos.0 = min_point.0;
        }
        if min_point.1 < min_pos.1 {
            min_pos.1 = min_point.1;
        }
        if max_point.0 > max_pos.0 {
            max_pos.0 = max_point.0;
        }
        if max_point.1 > max_pos.1 {
            max_pos.1 = max_point.1;
        }
    }

    println!("Min {min_pos:?}");
    println!("Max {max_pos:?}");

    // let part1 = exercise_one(&sensors, min_pos, max_pos);
    let part2 = exercise_two(&sensors, 4000000);
    assert!(part2 != -1);
    // println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
