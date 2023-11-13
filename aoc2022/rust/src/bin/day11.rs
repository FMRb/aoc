use std::collections::VecDeque;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult,
    multi::separated_list1,
    bytes::complete::{tag, take_until},
    character::{complete::{newline, multispace1, self, digit1}, streaming::space1},
    sequence::terminated, branch::alt
};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multi,
}

#[derive(Debug, Clone)]
enum Operand {
    Num(u64),
    Old,
}

type Operation = (Operand, Operator, Operand);

#[derive(Debug, Copy, Clone)]
struct Test {
    divided_by: u64,
    true_throw: u64,
    false_throw: u64,
}

#[derive(Debug)]
struct Monkey {
    objects: VecDeque<u64>,
    operation: Operation, 
    test: Test, 
    count_inspections: u64,
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Self { 
            objects: self.objects.clone(),
            operation: self.operation.clone(),
            test: self.test.clone(),
            count_inspections: self.count_inspections.clone()
        }
    }
}

impl Monkey {
    fn new(objects: VecDeque<u64>, operation: Operation, test: Test) -> Self {
        Self {
            objects,
            operation,
            test,
            count_inspections: 0,
        }
    }

    fn take_object(&mut self) -> Option<u64> {
        self.objects.pop_front()
    }

    fn add_object(&mut self, object: u64) {
        self.objects.push_back(object);
    } 

    fn operate_object(&self, object: u64, reminder: u64) -> u64 {
        let (operand_lhs, operator, operand_rhs) = &self.operation;
        let operand_lhs = match operand_lhs {
            Operand::Num(value) => *value as u64,
            Operand::Old => object,
        };
        let operand_rhs = match operand_rhs {
            Operand::Num(value) => *value as u64,
            Operand::Old => object,
        };

        match operator {
            Operator::Add => (operand_lhs + operand_rhs) % reminder,
            Operator::Multi => (operand_lhs * operand_rhs) % reminder,
        } 
    }

    fn inspect_object_default_relief(&mut self, reminder: u64) -> Option<(u64, u64)>{
        if let Some(object) = self.take_object() {
            self.count_inspections += 1;
            let worry_level = self.operate_object(object, reminder);
            let worry_level = worry_level / 3;
            if worry_level % self.test.divided_by as u64 == 0 {
                return Some((worry_level, self.test.true_throw));
            } else {
                return Some((worry_level, self.test.false_throw));
            }
        }
        None
    }
    
    fn inspect_object_relief(&mut self, reminder: u64) -> Option<(u64, u64)>{
        if let Some(object) = self.take_object() {
            self.count_inspections += 1;
            let worry_level = self.operate_object(object, reminder);
            if worry_level % self.test.divided_by == 0 {
                return Some((worry_level, self.test.true_throw));
            } else {
                return Some((worry_level, self.test.false_throw));
            }
        }
        None
    }

    fn inspecting(&mut self, relief_default: bool, reminder: u64) -> Vec<(u64, u64)> {
        let mut throw_objects = Vec::new(); 

        if relief_default {
            while let Some(throw_object) = self.inspect_object_default_relief(reminder) {
                throw_objects.push(throw_object);
            }
        } else {
            while let Some(throw_object) = self.inspect_object_relief(reminder) {
                throw_objects.push(throw_object);
            }
        }
        throw_objects
    }
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = multispace1(input)?;
    let (input, _) = terminated(take_until("items: "), tag("items: "))(input)?;
    let (input, items) = separated_list1(tag(", "), digit1)(input)?;
    let items = items
        .iter()
        .flat_map(|item| item.parse::<u64>().ok())
        .collect();
    let (input, _) = newline(input)?;
    Ok((input, items))
}

fn match_operand(operand: &str) -> Operand {
    if operand == "old" {
        return Operand::Old;
    }

    let value = operand.parse::<u64>().unwrap();
    Operand::Num(value)
}
fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = multispace1(input)?;
    let (input, _) = terminated(take_until(" = "), tag(" = "))(input)?;
    let (input, lhs) = alt((tag("old"), digit1))(input)?;
    let (input, _) = space1(input)?;
    let (input, operator) = alt((tag("+"), tag("*")))(input)?;
    let (input, _) = space1(input)?;
    let (input, rhs) = alt((tag("old"), digit1))(input)?;
    let (input, _) = newline(input)?;

    let lhs = match_operand(lhs);
    let rhs = match_operand(rhs);
    let operator = match operator {
        "+" => Operator::Add,
        "*" => Operator::Multi,
        _ => panic!("Wrong operator")
    };

    Ok((input, (lhs, operator, rhs)))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, _) = multispace1(input)?;
    let (input, _) = terminated(take_until("by "), tag("by "))(input)?;
    let (input, divided_by) = complete::u64(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = terminated(take_until("monkey "), tag("monkey "))(input)?;
    let (input, true_throw) = complete::u64(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = terminated(take_until("monkey "), tag("monkey "))(input)?;
    let (input, false_throw) = complete::u64(input)?;

    Ok((input, Test{ divided_by, true_throw, false_throw }))
}


fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // Disregard monkey index, the index is determined by the position in the vec and order of parse
    let (input, _) = terminated(take_until("\n"), newline)(input)?;
    let (input, objects) = parse_starting_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, test) = parse_test(input)?;

    Ok((input, Monkey::new(VecDeque::from(objects), operation, test)))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input)?;
    Ok((input, monkeys))
}


fn run_game(rounds: u64, monkeys: &mut Vec<Monkey>, relief_default: bool, reminder: u64) {
    // 20 rounds
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let move_items = monkeys[i]
                .inspecting(relief_default, reminder);
            for (item, index_monkey) in move_items {
                monkeys[index_monkey as usize].add_object(item);
            }
        }
    }
}


fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/11.input")?;
    let (_, monkeys) = parse_monkeys(&input).unwrap();

    let reminder: u64 = monkeys
        .iter()
        .map(|m| m.test.divided_by)
        .product();

    // Part 1
    let mut p1_monkeys = Vec::clone(&monkeys);
    run_game(20, &mut p1_monkeys, true, reminder);

    let monkey_business_p1: Vec<u64> = p1_monkeys
        .iter()
        .map(|m| m.count_inspections)
        .collect();
    println!("{:?}", monkey_business_p1);
        
    let mut p2_monkeys = Vec::clone(&monkeys);

    run_game(10000, &mut p2_monkeys, false, reminder);

    p2_monkeys
        .sort_by_key(|m| m.count_inspections);
    
    let monkey_business_p2: u64 = p2_monkeys
        .iter()
        .rev()
        .map(|m| m.count_inspections)
        .take(2)
        .product();
        
        // .map(|m| m.count_inspections)
        // .take(2)
        // .product();

    // let result = monkey_business[0] * monkey_business[1];
    // println!("Part 1: {result}");
    println!("Part 2: {monkey_business_p2}");
    Ok(())
}
