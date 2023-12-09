use std::collections::HashMap;
use std::vec;
use regex::Regex;
use gloo::console::log;
use std::slice::Iter;
use std::str::FromStr;
use std::iter;

use rand::distributions::{Distribution, Uniform};

use wasm_bindgen::prelude::*;
#[cfg(feature = "web_build")]
pub use wasm_bindgen_rayon::init_thread_pool;
use rayon::prelude::*;
mod dice_parser;


////////////////////////////////////////////
/// Enums
////////////////////////////////////////////
#[cfg_attr(feature = "web_build", wasm_bindgen)]
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
    pub fn compare(&self, a: usize, b: usize) -> bool {
        match self {
            Comparison::LessEqual => return a <= b,
            Comparison::GreaterEqual => return a >= b,
            Comparison::LessThan => return a < b,
            Comparison::GreaterThan => return a > b,
            Comparison::Equal => return a == b,
        }
    }
    pub fn iter() -> Iter<'static, Comparison> {
        static DIRECTIONS: [Comparison; 5] = [
            Comparison::LessEqual,
            Comparison::GreaterEqual,
            Comparison::LessThan,
            Comparison::GreaterThan,
            Comparison::Equal,
        ];
        DIRECTIONS.iter()
    }
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
pub fn do_comparison(op: Comparison, a: usize, b: usize) -> bool {
    // wasm doesn't like function for enums
    return op.compare(a, b);
}

////////////////////////////////////////////
/// Structs
////////////////////////////////////////////
#[cfg_attr(feature = "web_build", wasm_bindgen)]
#[derive(Clone, PartialEq)]
pub struct Dice {
    sides: usize,
    distribution: Uniform<usize>,
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
#[derive(Clone, PartialEq)]
pub struct DiceSet {
    dice: Vec<Dice>,
}


////////////////////////////////////////////
/// Impl
////////////////////////////////////////////
#[cfg_attr(feature = "web_build", wasm_bindgen)]
#[derive(Clone, PartialEq)]
pub struct Decision {
    operator: Comparison,
    decision_dice: DiceSet,
    decision_value: usize,
    dice: DiceSet,
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
#[derive(Clone, PartialEq)]
pub struct DecisionSet {
    decisions: Vec<Decision>
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

#[cfg_attr(feature = "web_build", wasm_bindgen)]
impl Dice {

    #[cfg_attr(feature = "web_build", wasm_bindgen(constructor))]
    pub fn new(sides: usize) -> Dice {
        let distribution = Uniform::from(1..sides);
        return Dice {
            sides,
            distribution,
        };
    }


    #[cfg_attr(feature = "web_build", wasm_bindgen(setter))]
    pub fn set_sides(&mut self, sides: usize) {
        self.sides = sides;
    }

    #[cfg_attr(feature = "web_build", wasm_bindgen(getter))]
    pub fn sides(&self) -> usize {
        return self.sides;
    }
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
impl DiceSet {

    #[cfg_attr(feature = "web_build", wasm_bindgen)]
    pub fn empty() -> DiceSet {
        return DiceSet { dice: vec![] };
    }

    #[cfg_attr(feature = "web_build", wasm_bindgen)]
    pub fn from_string(text: &str) -> DiceSet {
        let re = Regex::new(r"(?<num_dice>\d+)d(?<num_sides>\d+)()").unwrap();
        let caps = re
        .captures_iter(&text)
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
        // #[cfg(feature = "web_build")]
        // log!("parsed", text.clone(), "into", caps.to_string());

        return caps;
    }

    #[cfg_attr(feature = "web_build", wasm_bindgen)]
    pub fn to_str(&self) -> String {
        return self.to_string();
    }
    
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
impl Decision {

    #[cfg_attr(feature = "web_build", wasm_bindgen(constructor))]
    pub fn new(
        operator: Comparison,
        decision_dice: DiceSet,
        decision_value: usize,
        dice: DiceSet) -> Decision {
            // log!("creating decision", operator.clone());
            return Decision {
                operator: operator,
                decision_dice: decision_dice,
                decision_value: decision_value,
                dice: dice,
            }
        }

    #[cfg_attr(feature = "web_build", wasm_bindgen(getter))]
    pub fn operator(&self) -> Comparison {
        return self.operator.clone();
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(setter))]
    pub fn set_operator(&mut self, op: Comparison) {
        self.operator = op;
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(getter))]
    pub fn decision_dice(&self) -> DiceSet{
        return self.decision_dice.clone();
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(setter))]
    pub fn set_decision_dice(&mut self, dice: &DiceSet) {
        self.decision_dice = dice.clone();
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(getter))]
    pub fn decision_value(&self) -> usize {
        return self.decision_value;
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(setter))]
    pub fn set_decision_value(&mut self, value: usize) {
        self.decision_value = value;
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(getter))]
    pub fn dice(&self) -> DiceSet {
        return self.dice.clone();
    }
    #[cfg_attr(feature = "web_build", wasm_bindgen(setter))]
    pub fn set_dice(&mut self, dice: &DiceSet) {
        self.dice = dice.clone();
    }
    
}

// #[cfg_attr(feature = "web_build", wasm_bindgen)]
// impl DecisionSet {
//     #[cfg_attr(feature = "web_build", wasm_bindgen(getter))]
//     pub fn decisions(&self) -> Vec<Decision> {
//         return self.decisions.clone();
//     }

//     #[cfg_attr(feature = "web_build", wasm_bindgen(setter))]
//     pub fn set_decisions(&mut self, decisions: &Vec<Decision>) {
//         self.decisions = decisions.clone();
//     }
// }


////////////////////////////////////////////
/// Rollable
////////////////////////////////////////////
pub trait Rollable : Sync {
    fn roll(&self, verbose: bool) -> usize;
    fn max(&self) -> usize;
    fn to_string(&self) -> String;
}

impl Rollable for Dice {
    fn roll(&self, verbose: bool) -> usize {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let sample = self.distribution.sample(&mut rng);
        // if verbose {
        //     #[cfg(feature = "web_build")]
        //     log!(self.to_string(), "rolled a", sample);
        // }
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


impl Rollable for DiceSet {
    fn roll(&self, verbose: bool) -> usize {
        let mut value: usize = 0;
        for die in &self.dice {
            value += die.roll(verbose);
        }
        // if verbose {
        //     #[cfg(feature = "web_build")]
        //     log!(self.to_string(), "total value", value);
        // }

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

impl Rollable for Decision {
    fn roll(&self, verbose: bool) -> usize {
        let decision_roll = self.decision_dice.roll(verbose);
        let should_roll = self.operator.compare(decision_roll, self.decision_value);
        // if verbose {
        //     #[cfg(feature = "web_build")]
        //     log!(self.to_string(), "should roll dice", should_roll);
        // }
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
        // if verbose {
        //     #[cfg(feature = "web_build")]
        //     log!(self.to_string(), "total value", value);
        // }

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

////////////////////////////////////////////
/// helper functions
////////////////////////////////////////////

pub fn run_sim(dice: &dyn Rollable, iters: i32) -> Vec<f64> {

    let mut max = dice.max();
    if max == 0 {
        max = 1;
    }
    let mut hist = vec![0.0; max];
    // #[cfg(feature = "web_build")]
    // log!("rolling", dice.to_string(), "with max", max);
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

pub fn run_sim_parallel(dice: &dyn Rollable, iters: i32) -> Vec<f64> {
    #[cfg(feature = "web_build")]
    log!("inside run sim");
    let num_threads = rayon::current_num_threads();
    #[cfg(feature = "web_build")]
    log!("using threadpool size ", num_threads);
    let pool_result = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build();

    if pool_result.is_err() {
        let _err = pool_result.err().unwrap();
        
        #[cfg(feature = "web_build")]
        log!("failed to create thread pool", _err.to_string());
        return vec![];
    }
    let pool = pool_result.unwrap();

    #[cfg(feature = "web_build")]
    log!("got pool");
    let iters_per_thread = iters / (num_threads as i32);
    let result = pool.broadcast(|_| run_sim(dice.clone(), iters_per_thread));
    #[cfg(feature = "web_build")]
    log!("got results");
    if result.len() == 0 {
        return vec![];
    }
    let mut combined: Vec<f64> = iter::repeat(0 as f64).take(result.get(0).unwrap().len()).collect();
    for vals in result {
        for i in 0..vals.len() {
            combined[i] += vals[i];
        }
    }

    return combined;
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
pub fn run_sim_decision(decision: Decision, iters: i32) -> Vec<f64> {
    // 16025.521
    #[cfg(feature = "web_build")]
    log!("calling run sim");
    // return run_sim_parallel(&decision, iters);
    return run_sim(&decision, iters);
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
pub fn get_valid_dice(dice: String) -> String {
    // let mut ret = String::new();
    // let mut has_d = false;
    // for char in dice.chars() {
    //     if char.is_numeric() {
    //         // log!("pushing", char.to_string());
    //         ret.push(char);
    //     } else if char == 'd' && !has_d && ret.len() > 0 {
    //         // log!("pushing dice", char.to_string());
    //         ret.push(char);
    //         has_d = true;
    //     } else if char == ',' && has_d {
    //         // log!("pushing comma", char.to_string());
    //         ret.push(char);
    //         has_d = false;
    //     }
    // }

    // return ret;
    return dice;
}

#[cfg_attr(feature = "web_build", wasm_bindgen)]
pub fn parse_dice(value: String) -> DiceSet {
    let re = Regex::new(r"(?<num_dice>\d+)d(?<num_sides>\d+)(?<test>\+\d+){0,1}").unwrap();
    let caps = re
        .captures_iter(&value)
        .map(|x| {
            let num_dice = x["num_dice"].parse::<usize>().unwrap();
            let dice_sides = x["num_sides"].parse::<usize>().unwrap();
            // log!(x["test"].parse());
            if dice_sides > 1 {
                return DiceSet {
                    dice: vec![Dice::new(dice_sides); num_dice],
                }; 
            } else {
                return DiceSet::empty();
            }
        })
        .collect::<DiceSet>();
    
    #[cfg(feature = "web_build")]
    log!("parsed", value.clone(), "into", caps.to_string());

    return caps;
}

 
pub fn get_histogram(dice_str: String, iters: i32) -> Vec<f64> {
    let dice = parse_dice(dice_str);
    return run_sim(&dice, iters);
}


#[test]
fn test_parallel() {
    let d = parse_dice("20d20".to_string());
    run_sim_parallel(&d, 1000000);
    // run_sim(&d, 100);
}

#[test]
fn test_sequential() {
    let d = parse_dice("20d20".to_string());
    run_sim(&d, 1000000);
}


#[test]
fn test_decision_parallel() {
    let d = parse_dice("1d20".to_string());
    let set = Decision::new(
        Comparison::GreaterThan, 
        d.clone(),
        12,
        d.clone()
    );
    run_sim_parallel(&set, 1000000);
    // run_sim(&d, 100);
}
