use std::cmp::Ordering;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
struct List {
    items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
enum Item {
    List(List),
    Value(u32),
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    let (input, items) =
        delimited(tag("["), separated_list0(tag(","), parse_item), tag("]"))(input)?;

    Ok((input, Item::List(List { items })))
}

fn parse_value(input: &str) -> IResult<&str, Item> {
    let (input, value) = complete::u32(input)?;
    Ok((input, Item::Value(value)))
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    let (input, item) = alt((parse_value, parse_list))(input)?;
    Ok((input, item))
}

fn parse_pair_packet(input: &str) -> IResult<&str, (Item, Item)> {
    let (input, one_packet) = parse_item(input)?;
    let (input, _) = newline(input)?;
    let (input, two_packet) = parse_item(input)?;
    Ok((input, (one_packet, two_packet)))
}

fn parse_packets(input: &str) -> IResult<&str, Vec<(Item, Item)>> {
    let (input, result) = separated_list1(tag("\n\n"), parse_pair_packet)(input)?;
    Ok((input, result))
}

fn compare_packets(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Item::List(left_list), Item::List(right_list)) => {
                    for (i, item) in left_list.items.iter().enumerate() {
                        let right_item = match right_list.items.get(i) {
                            Some(r_i) => r_i,
                            None => return Ordering::Greater
                        };
                        // println!("LIST: {item:?} {right_item:?}");

                        let compare_result = compare_packets(item, right_item);
                        if Ordering::is_ne(compare_result) {
                            return compare_result;
                        }
                    }

                    if left_list.items.len() == right_list.items.len() {
                        Ordering::Equal
                    } else if left_list.items.len() < right_list.items.len() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
        },
        (Item::List(left_list), Item::Value(r_v)) => {
                compare_packets(
                    left,
                    &Item::List(List {
                        items: vec![Item::Value(*r_v)],
                    }),
                )
            
        },
        (Item::Value(l_v), Item::List(right_list)) => {
                compare_packets(
                    &Item::List(List {
                        items: vec![Item::Value(*l_v)],
                    }),
                    right,
                )
            }
        (Item::Value(l_v), Item::Value(r_v)) if l_v == r_v => Ordering::Equal,
        (Item::Value(l_v), Item::Value(r_v)) if l_v < r_v => Ordering::Less,
        (Item::Value(_l_v), Item::Value(_r_v)) => Ordering::Greater,
    }
}

fn exercise_one(packets: &[(Item, Item)]) -> u32 {
    let mut correct_pairs = 0;
    for (i, (left, right)) in packets.iter().enumerate() {
        if Ordering::is_lt(compare_packets(left, right)){
                // println!("INDEX: {i}");
                // println!("left: {left:?}");
                // println!("right: {right:?}");
                // println!("---------------");
                correct_pairs += i + 1;
        }
    }
    correct_pairs as u32
}

fn exercise_two(packets: Vec<(Item, Item)>) -> u32 {
    let mut all_packets = Vec::new();

    let divider_packet_one = Item::List(
        List{ 
            items: vec![Item::List(List{ items: vec![Item::Value(2)]})]
        }
    );
    let divider_packet_two = Item::List(
        List{ 
            items: vec![Item::List(List{ items: vec![Item::Value(6)]})]
        }
    );
    let dpo = divider_packet_one.clone();
    let dpt = divider_packet_two.clone();
    all_packets.push(divider_packet_one);
    all_packets.push(divider_packet_two);
    for (left, right) in packets {
        all_packets.push(left);
        all_packets.push(right)
    }

    all_packets.sort_by(|a, b| {
        compare_packets(a, b)
    });


    let dpo_pos = all_packets.iter().position(|packet| {
        matches!(compare_packets(packet, &dpo), Ordering::Equal)
    }).unwrap();
    let dpt_pos = all_packets.iter().position(|packet| {
        matches!(compare_packets(packet, &dpt), Ordering::Equal)
    }).unwrap();
    
    ((dpo_pos + 1) * (dpt_pos + 1)) as u32
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/13.input")?;
    let (_input, packets) = parse_packets(&input).unwrap();

    let part1 = exercise_one(&packets);
    let part2 = exercise_two(packets);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}
