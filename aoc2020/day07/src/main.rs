use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

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
#[derive(Debug, Hash)]
struct Bag {
    color: String,
    contents: Vec<(String, u32)>
}

impl Bag {
    fn new(color: String, ) -> Self {
        let contents:Vec<(String, u32)> = Vec::new();
        Self {
            color,
            contents
        }
    }

    fn add_bag(&mut self, color: String, quantity: u32) {
        self.contents.push((color, quantity));
    }

    fn contains(&self, search: &str) -> bool {
        if let Some(_b) = self.contents.iter().find(|&(color, _q)| {
            return *color == search
        }) {
            return true
        } else {
            false
        }
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl Eq for Bag {}

// Example
// clear plum bags contain 3 dark red bags, 3 dim gold bags, 2 posh black bags, 5 plaid beige bags.
// clear plum bags contain no other bags.
fn parse_input(input: &str) -> HashMap<String, Bag>{
    let mut bag_map:HashMap<String, Bag> = HashMap::new();

    for line in input.lines() {
        let mut rules:Vec<&str> = line
            .split(" bags contain ")
            .collect();

        rules.reverse();
        let mut bag = Bag::new(String::from(rules.pop().unwrap()));
        if rules[0] == "no other bags." {
            bag_map.insert(String::from(&bag.color), bag);
            continue;
        }

        let split_rules = rules[0]
            .split(',')
            .filter(|rule| !rule.is_empty())
            .map(|rule| rule.trim());

        for rule in split_rules {
            let m = rule
                .splitn(2, ' ')
                .collect::<Vec<&str>>();

            let quantity = m[0]
                .parse::<u32>()
                .unwrap();

            let color = m[1]
                .splitn(3, ' ')
                .take(2)
                .collect::<Vec<&str>>()
                .join(" ");
            bag.add_bag(color, quantity);
        }
        bag_map.insert(String::from(&bag.color), bag);
    }

    bag_map
}

fn find_number_bags(bags: &HashMap<String,Bag>, searching_for: &str, hs: &mut HashSet<String>) {
    for bag in bags.values() {
        if bag.contains(searching_for) {
            hs.insert(String::from(&bag.color));
            find_number_bags(&bags, &bag.color, hs);
        }
    }
}

fn part_one(input: &str) -> u32 {
    let bags = parse_input(input);
    let searching_for = "shiny gold";

    let mut hs:HashSet<String> = HashSet::new();
    find_number_bags(&bags, searching_for, &mut hs);
    hs.len() as u32
}

fn find_bags(bags: &HashMap<String,Bag>, seek_bag: &Bag) -> u32{
    let mut num = 0;
    if seek_bag.contents.len() == 0 {
        return num;
    }

    for (color, quantity) in &seek_bag.contents {
        let new_seek_bag = bags.get(color).unwrap();
        num += quantity + (quantity * find_bags(bags, &new_seek_bag));
    }
    num
}

fn part_two(input: &str) -> u32 {
    let bags = parse_input(input);
    let starting_bag = bags.get(&String::from("shiny gold")).unwrap();
    find_bags(&bags, &starting_bag)
}