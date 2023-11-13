use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Game {
    Rock,
    Paper,
    Scissors
}

impl Game {
    fn getValue(&self) -> u32 {
        match self {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }

    fn winning(&self) -> Game {
        if *self == Game::Rock {
            return Game::Scissors;
        }

        if *self == Game::Paper {
            return Game::Rock;
        }

        return Game::Paper;
    }

    fn losing(&self) -> Game {
        if *self == Game::Rock {
            return Game::Paper;
        }

        if *self == Game::Paper {
            return Game::Scissors;
        }

        Game::Rock
    }

    fn compete(&self, other: &Game) -> u32 {
        if self == other {
            return 3; 
        }

        if self.winning() == *other {
            return 6
        }

        0
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let result = match s {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" => Game::Scissors,
            "X" => Game::Rock,
            "Y" => Game::Paper,
            "Z" => Game::Scissors,
            _ => unreachable!("no valid input")
        };
        Ok(result)
    }
}

fn parse_game(input: &str) -> Game {
    match input {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" => Game::Scissors,
            "X" => Game::Rock,
            "Y" => Game::Paper,
            "Z" => Game::Scissors,
            _ => unreachable!("no valid input")
        }
}

fn exercise_one() -> u32 {
    let input: Vec<(Game, Game)> = include_str!("../../data/2.input")
        .lines()
        .map(|line| {
            let (o, p ) = match line.split_once(" ") {
                Some((o, p)) => (o, p),
                None => panic!("Invalid error"),
            };
            (parse_game(o), parse_game(p))
        })
        .collect();
    
    let mut score = 0;
    for (o, p) in input {
        score += p.compete(&o) + p.getValue();
    }
    score
}

fn exercise_two() -> u32 {
    let input: Vec<(Game, Game)> = include_str!("../../data/2.input")
        .lines()
        .map(|line| {
            let (o, p ) = match line.split_once(" ") {
                Some((o, p)) => (o, p),
                None => panic!("Invalid error"),
            };
            let o = parse_game(&o);
            let p = match p {
                "X" => o.winning(),
                "Y" => o,
                "Z" => o.losing(),
                _ => panic!("Invalid input")
            };
            (o, p)
        })
        .collect();
    
    let mut score = 0;
    for (o, p) in input {
        score += p.compete(&o) + p.getValue();
    }
    score
}

fn main() -> Result<()> {

    let score_one = exercise_one();
    let score_two = exercise_two();
    println!("Part 1 {:?}", score_one);
    println!("Part 2 {:?}", score_two);

    Ok(())
}