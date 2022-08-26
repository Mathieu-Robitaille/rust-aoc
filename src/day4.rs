use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HGT_REGEX: Regex = {
        let pattern = Regex::new(r"(\d+)(in|cm)").unwrap();
        pattern
    };
    static ref HCL_REGEX: Regex = {
        let pattern = Regex::new(r"^#([0-9a-f]){6}$").unwrap();
        pattern
    };
    static ref ECL_REGEX: Regex = {
        let pattern = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        pattern
    };
    static ref PID_REGEX: Regex = {
        let pattern = Regex::new(r"^([0-9]){9}$").unwrap();
        pattern
    };
}


#[derive(Debug, PartialEq)]
struct Hgt {
    unit: String,
    quant: usize,
}

impl Hgt {
    fn parse (input: &str) -> Option<Hgt> {
        match HGT_REGEX.captures(input) {
            None => None,
            Some(captures) => {
                let h = Hgt {
                    unit: String::from(captures.get(2).unwrap().as_str()),
                    quant: str::parse(captures.get(1).unwrap().as_str())
                        .expect("Unable to parse number"),
                };
                Some(h)
            }
        }
    }
    fn validate(&self) -> bool {
        match self.unit.as_str() {
            "cm" => self.quant >= 150 && self.quant <= 193,
            "in" => self.quant >= 59 && self.quant <= 76,
            invalid_height => {
                println!("Invalid quant | {:?} |", invalid_height);
                panic!("Quitting.");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Hgt>,
    hgt_string: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hgt_string: None,
            hcl: None,
            ecl: None,
            pid: None,
        }
    }

    fn valid_num_fields(&self) -> bool {

        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some() &&
        self.hgt_string.is_some()

    }

    fn validate(&self) -> bool {
        return 
            self.validate_byr() &&
            self.validate_iyr() &&
            self.validate_eyr() &&
            self.validate_hgt() &&
            self.validate_hcl() &&
            self.validate_ecl() &&
            self.validate_pid();
    }

    fn validate_byr(&self) -> bool {        
        match self.byr {
            Some(x) => x >= 1920 && x <= 2002,
            None => false,
        }
    }

    fn validate_iyr(&self) -> bool {        
        match self.iyr {
            Some(x) => x >= 2010 && x <= 2020,
            None => false,
        }
    }

    fn validate_eyr(&self) -> bool {        
        match self.eyr {
            Some(x) => x >= 2020 && x <= 2030,
            None => false,
        }
    }

    fn validate_hgt(&self) -> bool {        
        match &self.hgt {
            Some(x) => x.validate(),
            None => false,
        }
    }

    fn validate_hcl(&self) -> bool {        
        match &self.hcl {
            Some(x) => HCL_REGEX.is_match(x.as_str()),
            None => false,
        }
    }
    
    fn validate_ecl(&self) -> bool {        
        match &self.ecl {
            Some(x) => ECL_REGEX.is_match(x.as_str()),
            None => false,
        }
    }

    fn validate_pid(&self) -> bool {        
        match &self.pid {
            Some(x) => PID_REGEX.is_match(x.as_str()),
            None => false,
        }
    }

}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|line| gen_passport(line))
        .collect()
}

fn gen_passport(line: &str) -> Passport {
    let input: Vec<&str> = line
        .lines()
        .flat_map(|line| line.split(" "))
        .collect();

    let mut pp = Passport::new();

    for x in input {
        let entry: Vec<&str> = x.split(":").collect();

        match *(entry.get(0).unwrap()) {
            "byr" => pp.byr = Some(str::parse(*entry.get(1).unwrap()).unwrap()),
            "iyr" => pp.iyr = Some(str::parse(*entry.get(1).unwrap()).unwrap()),
            "eyr" => pp.eyr = Some(str::parse(*entry.get(1).unwrap()).unwrap()),
            "hgt" => {
                pp.hgt_string = Some(String::from(*entry.get(1).unwrap()));
                pp.hgt        = Hgt::parse(*entry.get(1).unwrap());
            },
            "hcl" => pp.hcl = Some(String::from(*entry.get(1).unwrap())),
            "ecl" => pp.ecl = Some(String::from(*entry.get(1).unwrap())),
            "pid" => pp.pid = Some(String::from(*entry.get(1).unwrap())),
            "cid" => (),
            invalid_passport => {
                println!("Oh no! This passport is bad | {:?} |", invalid_passport);
                panic!("Quitting");
            },
        }
    }
    return pp;
}

#[aoc(day4, part1)]
fn part_one(input: &Vec<Passport>) -> usize {
    input
        .iter()
        .filter(|entry| entry.valid_num_fields())
        .count()
}

#[aoc(day4, part2)]
fn part_two(input: &Vec<Passport>) -> usize {
    input
        .iter()
        .filter(|entry| entry.validate())
        .count()
}
