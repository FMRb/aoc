use std::collections::HashSet;
use std::env;
use std::fs;

use std::str::Lines;

fn main() -> Result<(), Box<dyn (std::error::Error)>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <path_to_input>");
        std::process::exit(1);
    }
    println!("Argument {}", args[1]);
    let path = String::from(&args[1]);
    let input = fs::read_to_string(&path)?;
    let p1 = part_one(&input);
    println!("Result part 1: {}", p1);
    let p2 = part_two(&input);
    println!("Result part 2: {}", p2);
    Ok(())
}

/*
// Input example
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
*/

type ValueRange = (u32, u32);
#[derive(Debug, PartialEq, Eq, Hash)]
struct FieldRule<'a> {
    name: &'a str,
    ranges: Vec<ValueRange>,
}

impl FieldRule<'_> {
    fn contain_number(&self, number: u32) -> bool {
        self.ranges
            .iter()
            .any(|&value| number >= value.0 && value.1 >= number)
    }

    fn check_validity(&self, values: &Vec<u32>) -> bool {
        values.iter().all(|&v| self.contain_number(v))
    }
}

fn parse_rules<'a>(input: &mut Lines<'a>) -> Vec<FieldRule<'a>> {
    let mut field_rules: Vec<FieldRule> = Vec::new();
    for line in input {
        if line.len() == 0 {
            break;
        }
        let split_name_values: Vec<&str> = line.split(':').collect();
        let name = *split_name_values.first().unwrap();
        let ranges = split_name_values.last().unwrap().split("or").fold(
            Vec::with_capacity(2),
            |mut acc, range| {
                let r: Vec<u32> = range
                    .trim()
                    .split('-')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect();

                let value_range: ValueRange = (r[0], r[1]);
                acc.push(value_range);
                acc
            },
        );
        field_rules.push(FieldRule { name, ranges })
    }
    field_rules
}

type Ticket = Vec<u32>;

fn parse_ticket(input: &str) -> Ticket {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

fn parse_my_ticket<'a>(input: &mut Lines<'a>) -> Ticket {
    // Avoid input line "your ticket:" (skip(1))
    parse_ticket(input.skip(1).next().unwrap())
}

fn parse_nearby_tickets<'a>(input: &mut Lines<'a>) -> Vec<Ticket> {
    // Skip(2) empty line + input line "nearby tickets:"
    input.skip(2).fold(Vec::new(), |mut tickets, line| {
        tickets.push(parse_ticket(line));
        tickets
    })
}

fn parse_valid_nearby_tickets<'a>(
    input: &mut Lines<'a>,
    field_rules: &Vec<FieldRule>,
) -> Vec<Ticket> {
    input.skip(2).fold(Vec::new(), |mut tickets, line| {
        let ticket = parse_ticket(line);
        if field_rules.iter().any(|fr| fr.check_validity(&ticket)) {
            tickets.push(ticket);
        }
        tickets
    })
}

fn part_one(input: &str) -> u32 {
    let mut input = input.lines();

    let field_rules = parse_rules(&mut input);
    let _my_ticket = parse_my_ticket(&mut input);
    let nearby_tickets = parse_nearby_tickets(&mut input);

    let mut invalid_values = Vec::new();
    for nearby_ticket in nearby_tickets {
        for value in nearby_ticket {
            if field_rules.iter().any(|fr| fr.contain_number(value)) {
                continue;
            }
            invalid_values.push(value);
        }
    }
    invalid_values.iter().sum()
}

fn part_two(input: &str) -> u64 {
    let mut input = input.lines();

    let field_rules = parse_rules(&mut input);
    let my_ticket = parse_my_ticket(&mut input);
    let valid_nearby_tickets = parse_valid_nearby_tickets(&mut input, &field_rules);

    let mut ordered_field_names: Vec<(u32, HashSet<&FieldRule>)> = Vec::new();
    for order in 0..field_rules.len() {
        let mut tickets_by_column: Vec<u32> = valid_nearby_tickets
            .iter()
            .map(|ticket| ticket[order])
            .collect();

        // Include my ticket to make the calculations
        tickets_by_column.push(my_ticket[order]);

        let field_set = field_rules
            .iter()
            .filter(|&fr| fr.check_validity(&tickets_by_column))
            .collect::<HashSet<_>>();

        ordered_field_names.push((order as u32, field_set));
    }

    ordered_field_names.sort_by_key(|(_, field_set)| field_set.len());

    let field_mapping = ordered_field_names
        .iter()
        .scan(HashSet::new(), |field_used, (order, field_set)| {
            let field = *field_set.difference(&field_used).next().unwrap();
            field_used.insert(field);
            Some((field, order))
        })
        .collect::<Vec<_>>();

    let result = field_mapping
        .iter()
        .filter_map(|&(field, order)| {
            if field.name.starts_with("departure") {
                Some(my_ticket[*order as usize] as u64)
            } else {
                None
            }
        })
        .product::<u64>();

    result
}
