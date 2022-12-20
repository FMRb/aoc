use std::{collections::{HashMap, VecDeque}, hash::Hash};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::{separated_list0, separated_list1},
    IResult,
};

fn bfs(valves: &[Valve], source: &Valve, target: &Valve) -> u64 {
    let mut visited = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_front(source);
    visited.push(source.id.to_string());
    let mut levels: HashMap<String, u64> = HashMap::new();

    levels.insert(source.id.to_string(), 0);

    while let Some(valve) = queue.pop_back() {
        for tunnel in valve.tunnels.iter() {
            if *tunnel == target.id {
                return *levels.get(&valve.id).unwrap() + 1;
            }
            levels.insert(tunnel.to_string(), *levels.get(&valve.id).unwrap() + 1);
            if !visited.contains(tunnel) {
                let new_valve = valves.iter().find(|&v| v.id == *tunnel).unwrap();
                queue.push_front(new_valve);
                visited.push(new_valve.id.to_string());
            }
        }
    }
    0
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Seen {
    location: String,
    remaining_steps: u64,
    open_valves: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    id: String,
    rate: u64,
    tunnels: Vec<String>,
}

fn find_highest_pressure<'a>(
    valves: &[Valve],
    source: &Valve,
    valves_closed: &'a Vec<Valve>,
    max_steps: u64,
) -> (u64, Vec<&'a Valve>) {
    let options = valves_closed.len();
    valves_closed
        .iter()
        .permutations(options)
        .map(|path| {
            let mut total = 0;
            let mut pressure = 0;
            let mut steps = 1;
            let mut source = source;
            let mut open_valves = Vec::new();
            for valve in path.iter() {
                let dist = bfs(valves, source, valve);
                source = valve;
                total += (pressure * dist) + (pressure + valve.rate);
                steps += dist + 1;
                pressure += valve.rate;
                open_valves.push(valve.id.to_string());

                if steps > max_steps {
                    break;
                }
            }
            if steps < max_steps {
                let remaining = max_steps - steps;
                total += remaining * pressure;
            }
            (total, path)
        })
        .max_by_key(|&(t, _)| t)
        .unwrap()
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve_id) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, rate) = complete::u64(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, tunnels) = separated_list0(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Valve {
            id: String::from(valve_id),
            rate,
            tunnels: tunnels.iter().map(|&t| String::from(t)).collect(),
        },
    ))
}

fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, parse_valve)(input)
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/16.example")?;
    let (_, valves) = parse_valves(&input).unwrap();

    for v in valves {
        let cost_map: HashMap<String, (Valve, u32)> = HashMap::new();
        bfs(&valves, &v);
    }
    let closed_valves: Vec<Valve> = valves.iter().filter(|v| v.rate > 0).cloned().collect();
    let source = valves.iter().find(|v| v.id.contains("AA")).unwrap();

    let max_steps = 30;
    let (total, p) = find_highest_pressure(&valves, source, &closed_valves, max_steps);
    println!("Path {p:?}");
    println!("Total {total}");
    Ok(())
}
