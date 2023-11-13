use std::env;
use std::fs;
use regex::Regex;

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

///////////// SAMPLE ///////////
// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm

// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929

// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm

// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in

#[derive(Debug)]
struct Passport {
    checked: Vec<String>,
    fields: Vec<String>
}

impl Passport {
    fn new() -> Self {
        let fields = Vec::with_capacity(8);
        let checked = Vec::with_capacity(8);
        Passport{
            fields,
            checked,
        }
    }

    fn process_birth_year(&mut self, byr: &str) {
        self.fields.push(String::from("byr"));
        match byr.parse::<u32>() {
            Ok(year) => {
                if year >= 1920 && year <= 2002 {
                    self.checked.push(String::from("byr"));
                }
            }
            Err(_) => {}
        }
    }

    fn process_issue_year(&mut self, iyr: &str) {
        self.fields.push(String::from("iyr"));
        match iyr.parse::<u32>() {
            Ok(year) => {
                if year >= 2010 && year <= 2020 {
                    self.checked.push(String::from("iyr"));
                }
            }
            Err(_) => {}
        }
    }

    fn process_expiration_year(&mut self, eyr: &str) {
        self.fields.push(String::from("eyr"));
        match eyr.parse::<u32>() {
            Ok(year) => {
                if year >= 2020 && year <= 2030 {
                    self.checked.push(String::from("eyr"));
                }
            }
            Err(_) => {}
        }
    }

    fn process_height(&mut self, height: &str) {
        self.fields.push(String::from("hgt"));
        let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        match re.captures(height) {
            Some(caps) => {
                if caps.get(1) == None {
                    return;
                }
                if caps.get(2) == None {
                    return;
                }

                let h = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let unit = caps.get(2).unwrap().as_str();

                if unit == "cm" && h >= 150 && h <= 193 {
                    self.checked.push(String::from("hgt"));
                    return;
                }

                if unit == "in" && h >= 59 && h <= 76 {
                    self.checked.push(String::from("hgt"));
                }
            }
            None => {}
        }
    }

    fn process_hair_color(&mut self, color: &str) {
        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        self.fields.push(String::from("hcl"));

        if re.is_match(color) {
            self.checked.push(String::from("hcl"));
        }
    }

    fn process_eye_color(&mut self, ecl: &str) {
        self.fields.push(String::from("ecl"));
        if ["amb","blu","brn","gry","grn","hzl","oth"].contains(&ecl) {
            self.checked.push(String::from("ecl"));
        }
    }

    fn process_passport_id(&mut self, pid: &str) {
        let re = Regex::new(r"^\d{9}$").unwrap();
        self.fields.push(String::from("pid"));
        if re.is_match(pid) {
            self.checked.push(String::from("pid"));
        }
    }

    fn simple_validation(&self) -> bool {
        self.fields.len() == 7
    }

    fn is_valid(&self) -> bool {
        self.checked.len() == 7
    }
}

fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports:Vec<Passport> = Vec::new();
    let mut passport = Passport::new();

    for line in input.lines() {
        // Start processing new passport
        if line.len() == 0 {
            passports.push(passport);
            passport = Passport::new();
            continue;
        }
        line
            .split_whitespace()
            .map(|field| field.split(':').collect::<Vec<&str>>())
            .for_each(|field| match field[0] {
                "byr" => {
                    passport.process_birth_year(field[1])
                }
                "iyr" => {
                    passport.process_issue_year(field[1])
                }
                "eyr" => {
                    passport.process_expiration_year(field[1])
                }
                "hgt" => {
                    passport.process_height(field[1])
                }
                "hcl" => {
                    passport.process_hair_color(field[1])
                }
                "ecl" => {
                    passport.process_eye_color(field[1])
                }
                "pid" => {
                    passport.process_passport_id(field[1])
                }
                _ => {}
            })
    }
    passports.push(passport);
    passports
}

fn part_one(input: &str) -> u32 {
    let passports = parse_passports(input);

    let valid_passports = passports
        .iter()
        .filter(|passport| passport.simple_validation())
        .count();

    valid_passports as u32
}

fn part_two(input: &str) -> u32 {
    let passports = parse_passports(input);

    let valid_passports = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();

    valid_passports as u32
}
