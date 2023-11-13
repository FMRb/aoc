use std::env;
use std::fs;
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

/*
Example input:

0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb

*/

enum RuleType {
    Char(char),
    SubRules(),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RuleId {
    id: u64,
}

#[derive(Debug, Clone)]
struct RulePattern {
    rules: Vec<RuleId>
}

#[derive(Debug, Clone)]
struct RuleSelection {
    rules: Vec<RulePattern>
}

struct RuleSet {
    rules: HashMap<RuleId, RuleSelection>,
}

fn parse_rules(input: &str) -> Result<Vec<RuleSet>,String> {
    let mut rule_set = HashMap::new();
    for line in input.lines() {
        // TODO: break with empty line
        let rule_str: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        let rule_id = rule_str[0].parse::<u64>().map(|id| RuleId{id}).unwrap();
        let sub_rule = rule_str[1];

        let rule = if sub_rule.contains('|') {
            let ids = sub_rule
                .split('|')
                .fold(Vec::new()|mut rule_pattern, s| {
                    s.split_whitespace()
                        .flat_map(|n| n.parse::<u64>().ok())
                        .map(|id| RuleId{id})
                    })
                .(|)
                .collect::<Vec<_>>();
            Rule {
                id: rule_id,
                sub_rules: Some(ids),
                single_char: None,
                mode: RuleType::SubRules,
            }
        } else if sub_rule.contains('"') {
            let c = sub_rule.trim_matches('"').chars().nth(0);
            Rule {
                id: rule_id,
                sub_rules: None,
                single_char: c,
                mode: RuleType::Single,
            }
        } else {
            let sub_rules: Vec<u32> = sub_rule
                .split_whitespace()
                .flat_map(|s| s.parse::<u32>().ok())
                .collect();
            Rule {
                id: rule_id,
                sub_rules: Some(sub_rules),
                single_char: None,
                mode: RuleType::Root,
            }
        }
    }
    todo!();
}

fn part_one(input: &str) -> u32 {
    0
}

fn part_two(input: &str) -> u32 {
    0
}
