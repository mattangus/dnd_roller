use std::collections::HashMap;
use std::str::FromStr;
use std::vec;
use regex::Regex;
use gloo::console::log;

use rand::distributions::{Distribution, Uniform};
use wasm_bindgen::prelude::*;

pub trait Rollable {
    fn roll(&self, verbose: bool) -> usize;
    fn max(&self) -> usize;
    fn to_string(&self) -> String;
}

#[derive(Clone, PartialEq)]
pub struct Dice {
    sides: usize,
    distribution: Uniform<usize>,
}

#[derive(Clone, PartialEq)]
pub struct DiceSet {
    dice: Vec<Dice>,
}

impl Rollable for Dice {
    fn roll(&self, verbose: bool) -> usize {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let sample = self.distribution.sample(&mut rng);
        if verbose {
            log!(self.to_string(), "rolled a", sample);
        }
        return sample;
        // return 0;
    }

    fn max(&self) -> usize {
        return self.sides;
    }

    fn to_string(&self) -> String {
        return format!("d{}", self.sides);
    }
}

impl Dice {
    pub fn new(sides: usize) -> Dice {
        let distribution = Uniform::from(1..sides);
        return Dice {
            sides,
            distribution,
        };
    }
}

impl Rollable for DiceSet {
    fn roll(&self, verbose: bool) -> usize {
        let mut value: usize = 0;
        for die in &self.dice {
            value += die.roll(verbose);
        }
        if verbose {
            log!(self.to_string(), "total value", value);
        }

        return value;
    }

    fn max(&self) -> usize {
        let mut value: usize = 0;
        for die in &self.dice {
            value += die.sides;
        }

        return value;
    }

    fn to_string(&self) -> String {
        let mut sides_to_count = HashMap::new();

        for item in &self.dice {
            let count = sides_to_count.entry(item.sides).or_insert(0);
            *count += 1;
        }

        let values: Vec<String> = sides_to_count
            .iter()
            .map(|(sides, count)| {
                return format!("{}d{}", count, sides);
            })
            .collect();

        if values.len() == 0 {
            return "<empty DiceSet>".to_string();
        }

        return values.join(",");
    }
}

impl FromIterator<DiceSet> for DiceSet {
    fn from_iter<T: IntoIterator<Item = DiceSet>>(iter: T) -> Self {
        let mut dice = Vec::new();

        for item in iter {
            dice.extend(item.dice);
        }

        return DiceSet { dice };
    }
}

impl DiceSet {
    fn empty() -> DiceSet {
        return DiceSet { dice: vec![] };
    }
}

#[derive(Clone, PartialEq)]
pub enum Comparison {
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Equal,
}


impl FromStr for Comparison {
    type Err = ();

    fn from_str(input: &str) -> Result<Comparison, Self::Err> {
        match input {
            "<" => Ok(Comparison::LessThan),
            ">" => Ok(Comparison::GreaterThan),
            "<=" => Ok(Comparison::LessEqual),
            ">=" => Ok(Comparison::GreaterEqual),
            "=" => Ok(Comparison::Equal),
            "==" => Ok(Comparison::Equal),
            _ => Err(()),
        }
    }
}


impl ToString for Comparison {
    fn to_string(&self) -> String {
        let op = match self {
            Comparison::LessEqual => "<=",
            Comparison::GreaterEqual => ">=",
            Comparison::LessThan => "<",
            Comparison::GreaterThan => ">",
            Comparison::Equal => "==",
        };

        return op.to_string();
    }
}

impl Comparison {
    fn compare(&self, a: usize, b: usize) -> bool {
        match self {
            Comparison::LessEqual => return a <= b,
            Comparison::GreaterEqual => return a >= b,
            Comparison::LessThan => return a < b,
            Comparison::GreaterThan => return a > b,
            Comparison::Equal => return a == b,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Decision {
    operator: Comparison,
    decision_dice: DiceSet,
    decision_value: usize,
    dice: DiceSet,
}

#[derive(Clone, PartialEq)]
pub struct DecisionSet {
    decisions: Vec<Decision>
}

impl Rollable for Decision {
    fn roll(&self, verbose: bool) -> usize {
        let decision_roll = self.decision_dice.roll(verbose);
        let should_roll = self.operator.compare(decision_roll, self.decision_value);
        if verbose {
            log!(self.to_string(), "should roll dice", should_roll);
        }
        if  should_roll {
            return self.dice.roll(verbose);
        }
        return 0;
    }

    fn max(&self) -> usize {
        return self.dice.max();
    }

    fn to_string(&self) -> String {
        return format!(
            "if {} {} {} then {}",
            self.decision_dice.to_string(),
            self.operator.to_string(),
            self.decision_value,
            self.dice.to_string()
        );
    }
}

// impl Decision {
//     fn empty() -> Decision {
//         return Decision {
//             operator: Comparison::LessThan,
//             decision_dice: DiceSet::empty(),
//             decision_value: 0,
//             dice: DiceSet::empty(),
//         };
//     }
// }

impl Rollable for DecisionSet {
    fn roll(&self, verbose: bool) -> usize {
        let mut value: usize = 0;
        for die in &self.decisions {
            value += die.roll(verbose);
        }
        if verbose {
            log!(self.to_string(), "total value", value);
        }

        return value;
    }

    fn max(&self) -> usize {
        let mut value: usize = 0;
        for die in &self.decisions {
            value += die.max();
        }

        return value;
    }

    fn to_string(&self) -> String {

        let values: Vec<String> = self.decisions
            .iter()
            .map(|d| {
                return d.to_string();
            })
            .collect();

        if values.len() == 0 {
            return "<empty DecisionSet>".to_string();
        }

        return values.join(",");
    }
}

pub fn run_sim(dice: &dyn Rollable, iters: i32) -> Vec<f64> {
    let mut max = dice.max();
    if max == 0 {
        max = 1;
    }
    let mut hist = vec![0.0; max];
    log!("rolling", dice.to_string(), "with max", max);
    for _ in 0..iters {
        let roll = dice.roll(false);
        hist[roll] += 1.0;
    }

    let mut sum: f64 = 0.0;
    for i in 0..hist.len() {
        sum += hist[i];
    }

    for i in 0..hist.len() {
        hist[i] /= sum;
    }

    return hist;
} 

#[wasm_bindgen]
pub fn get_valid_dice(dice: String) -> String {
    let mut ret = String::new();
    let mut has_d = false;
    for char in dice.chars() {
        if char.is_numeric() {
            // log!("pushing", char.to_string());
            ret.push(char);
        } else if char == 'd' && !has_d && ret.len() > 0 {
            // log!("pushing dice", char.to_string());
            ret.push(char);
            has_d = true;
        } else if char == ',' && has_d {
            // log!("pushing comma", char.to_string());
            ret.push(char);
            has_d = false;
        }
    }

    return ret;
}


pub fn parse_dice(value: &String) -> DiceSet {
    let re = Regex::new(r"(?<num_dice>\d+)d(?<num_sides>\d+)").unwrap();
    let caps = re
        .captures_iter(&value)
        .map(|x| {
            let num_dice = x["num_dice"].parse::<usize>().unwrap();
            let dice_sides = x["num_sides"].parse::<usize>().unwrap();
            if dice_sides > 1 {
                return DiceSet {
                    dice: vec![Dice::new(dice_sides); num_dice],
                };
            } else {
                return DiceSet::empty();
            }
        })
        .collect::<DiceSet>();

    log!("parsed", value.clone(), "into", caps.to_string());

    return caps;
}

#[wasm_bindgen]
pub fn get_histogram(dice_str: String, iters: i32) -> Vec<f64> {
    let dice = parse_dice(&dice_str);
    return run_sim(&dice, iters);
}