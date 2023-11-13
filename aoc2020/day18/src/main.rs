use std::env;
use std::fs;
use std::iter::Peekable;
use std::str::Chars;

fn main() -> Result<(), Box<dyn (std::error::Error)>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <path_to_input>");
        std::process::exit(1);
    }
    println!("Argument {}", args[1]);
    let path = String::from(&args[1]);
    let input = fs::read_to_string(&path)?;
    let p1 = part_one(&input)?;
    println!("Result part 1: {}", p1);
    let p2 = part_two(&input)?;
    println!("Result part 2: {}", p2);
    Ok(())
}

/*
1 + 2 * 3 + 4 * 5 + 6
  3   * 3 + 4 * 5 + 6
      9   + 4 * 5 + 6
         13   * 5 + 6
             65   + 6
                 71
*/

/*

http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp

 expr = expr SUM expr | expr PRODUCT expr | factor
 factor = INTLITERAL | LPAREN expr RPAREN


 Part 2

 expr = expr PRODUCT expr | term
 term = term SUM term | factor
 factor = INTLITERAL | LPAREN expr RPAREN
*/

#[derive(Debug, Clone)]
enum Grammar {
    Sum,
    Product,
    Number(isize),
}

#[derive(Debug, Clone)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: Grammar,
}

impl Node {
    fn new() -> Self {
        Self {
            left: None,
            right: None,
            value: Grammar::Number(0),
        }
    }

    fn bin_op(operation: Grammar, left: Box<Node>, right: Box<Node>) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            value: operation,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LexItem {
    Paren(char),
    Op(char),
    Num(isize),
}

fn parse_number(c: char, it: &mut Peekable<Chars>) -> isize {
    let mut s_number = c.to_string();

    while let Some(&digit) = it.peek() {
        match digit {
            '0'..='9' => {
                it.next();
                s_number.push(digit);
            }
            _ => {
                break;
            }
        }
    }

    s_number.parse::<isize>().expect("Expecting a digit")
}

type ParserFn<'a> = &'a dyn Fn(&Vec<LexItem>, usize) -> Result<(Node, usize), String>;

fn parse_tree(input: &str, parser: ParserFn) -> Result<Node, String> {
    let tokens = compute_lex(input)?;

    parser(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!("Expected input {:?} at {}", tokens[i], i))
        }
    })
}

// 1 + 2 + 3 + 4
//
/*
        expr
          +
        expr    3
       expr  + 2
      1
*/

fn parse_expr(tokens: &Vec<LexItem>, pos: usize) -> Result<(Node, usize), String> {
    let (mut node, mut next_pos) = parse_factor(tokens, pos)?;

    loop {
        let c = tokens.get(next_pos);
        match c {
            Some(&LexItem::Op('+')) | Some(&LexItem::Op('*')) => {
                let op = c.unwrap();
                let operation = if op == &LexItem::Op('+') {
                    Grammar::Sum
                } else {
                    Grammar::Product
                };

                let (factor, pos) = parse_factor(tokens, next_pos + 1)?;
                next_pos = pos;
                node = Node::bin_op(operation, Box::new(node), Box::new(factor));
            }
            _ => break,
        }
    }
    Ok((node, next_pos))
}

fn parse_factor(tokens: &Vec<LexItem>, pos: usize) -> Result<(Node, usize), String> {
    let c = tokens.get(pos);

    match c {
        Some(&LexItem::Num(n)) => {
            let mut node = Node::new();
            node.value = Grammar::Number(n);
            Ok((node, pos + 1))
        }
        Some(&LexItem::Paren(p)) => {
            if p != '(' {
                return Err(format!("Unexpected parenthesis {:?}", p));
            }

            let (node, next_pos) = parse_expr(tokens, pos + 1)?;
            Ok((node, next_pos + 1))
        }
        _ => Err(format!("Unexpected token {:?}", c)),
    }
}

fn compute_lex(input: &str) -> Result<Vec<LexItem>, String> {
    let mut lex = Vec::new();

    let mut it = input.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = parse_number(c, &mut it);
                lex.push(LexItem::Num(n));
            }
            '+' | '*' => {
                lex.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' => {
                lex.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }

    Ok(lex)
}

fn print(tree: &Node) -> String {
    match tree.value {
        Grammar::Sum => {
            let lhs = print(tree.left.as_ref().expect("sums need two children"));
            let rhs = print(tree.right.as_ref().expect("sums need two children"));
            format!("{} + {}", lhs, rhs)
        }
        Grammar::Product => {
            let lhs = print(tree.left.as_ref().expect("products need two children"));
            let rhs = print(tree.right.as_ref().expect("products need two children"));
            format!("{} * {}", lhs, rhs)
        }
        Grammar::Number(n) => format!("{}", n),
    }
}

fn compute(tree: &Node) -> isize {
    match tree.value {
        Grammar::Sum => {
            let lhs = compute(tree.left.as_ref().expect("sums need two children"));
            let rhs = compute(tree.right.as_ref().expect("sums need two children"));
            lhs + rhs
        }
        Grammar::Product => {
            let lhs = compute(tree.left.as_ref().expect("products need two children"));
            let rhs = compute(tree.right.as_ref().expect("products need two children"));
            lhs * rhs
        }
        Grammar::Number(n) => n,
    }
}

fn part_one(input: &str) -> Result<isize, String> {
    let mut result = 0;
    for line in input.lines() {
        let tree = parse_tree(line, &parse_expr)?;
        result += compute(&tree);
    }

    Ok(result)
}

fn parse_expr_v2(tokens: &Vec<LexItem>, pos: usize) -> Result<(Node, usize), String> {
    let (mut node, mut next_pos) = parse_term(tokens, pos)?;

    loop {
        let c = tokens.get(next_pos);
        match c {
            Some(&LexItem::Op('*')) => {
                let (term, pos) = parse_term(tokens, next_pos + 1)?;
                next_pos = pos;
                node = Node::bin_op(Grammar::Product, Box::new(node), Box::new(term));
            }
            _ => break,
        }
    }
    Ok((node, next_pos))
}

fn parse_term(tokens: &Vec<LexItem>, pos: usize) -> Result<(Node, usize), String> {
    let (mut node, mut next_pos) = parse_factor_v2(tokens, pos)?;

    loop {
        let c = tokens.get(next_pos);
        match c {
            Some(&LexItem::Op('+')) => {
                let (factor, pos) = parse_factor_v2(tokens, next_pos + 1)?;
                next_pos = pos;
                node = Node::bin_op(Grammar::Sum, Box::new(node), Box::new(factor));
            }
            _ => break,
        }
    }
    Ok((node, next_pos))
}

fn parse_factor_v2(tokens: &Vec<LexItem>, pos: usize) -> Result<(Node, usize), String> {
    let c = tokens.get(pos);

    match c {
        Some(&LexItem::Num(n)) => {
            let mut node = Node::new();
            node.value = Grammar::Number(n);
            Ok((node, pos + 1))
        }
        Some(&LexItem::Paren(p)) => {
            if p != '(' {
                return Err(format!("Unexpected parenthesis {:?}", p));
            }

            let (node, next_pos) = parse_expr_v2(tokens, pos + 1)?;
            Ok((node, next_pos + 1))
        }
        _ => Err(format!("Unexpected token {:?}", c)),
    }
}

fn part_two(input: &str) -> Result<isize, String> {
    let mut result = 0;
    for line in input.lines() {
        let tree = parse_tree(line, &parse_expr_v2)?;
        // println!("{:#?}", tree);
        result += compute(&tree);
    }

    Ok(result)
}
