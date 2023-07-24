use std::collections::HashMap;
use std::slice::Iter;
use std::str::FromStr;
use std::vec;

use chrono::format::Item;
use chrono::prelude::*;
use gloo::console::log;
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use web_sys::{EventTarget, HtmlTextAreaElement, HtmlSelectElement, HtmlInputElement};
use yew::prelude::*;
use yew::{html, Html, InputEvent};
use yew_bootstrap::component::form::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
use yew_plotly::plotly::common::Mode;
use yew_plotly::plotly::layout::themes::PLOTLY_DARK;
use yew_plotly::plotly::{Layout, Plot, Scatter};
use yew_plotly::Plotly;

pub trait Rollable {
    fn roll(&self, verbose: bool) -> usize;
    fn max(&self) -> usize;
    fn to_string(&self) -> String;
}

#[derive(Clone, PartialEq)]
struct Dice {
    sides: usize,
    distribution: Uniform<usize>,
}

#[derive(Clone, PartialEq)]
struct DiceSet {
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
    fn new(sides: usize) -> Dice {
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
enum Comparison {
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

#[derive(Clone, PartialEq)]
struct Decision {
    operator: Comparison,
    decision_dice: DiceSet,
    decision_value: usize,
    dice: DiceSet,
}

#[derive(Clone, PartialEq)]
struct DecisionSet {
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

impl Decision {
    fn empty() -> Decision {
        return Decision {
            operator: Comparison::LessThan,
            decision_dice: DiceSet::empty(),
            decision_value: 0,
            dice: DiceSet::empty(),
        };
    }
}

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

fn run_sim(dice: &dyn Rollable, iters: i32) -> Vec<f64> {
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

#[derive(Clone, PartialEq, Properties)]
struct TextProps {
    value: UseStateHandle<String>,
}

#[derive(Clone, PartialEq, Properties)]
struct DiceProps {
    die: UseStateHandle<DiceSet>,
}

#[derive(Clone, PartialEq, Properties)]
struct DiceCallbackProps {
    callback: Callback<DiceSet>,
}
#[derive(Clone, PartialEq, Properties)]
struct DecisionCallbackProps {
    callback: Callback<Decision>,
}

#[derive(Clone, PartialEq, Properties)]
struct DiceSetProps {
    die: UseStateHandle<Vec<DiceSet>>,
}

#[derive(Clone, PartialEq, Properties)]
struct DecisionProps {
    die: UseStateHandle<Decision>,
}

#[derive(Clone, PartialEq, Properties)]
struct DecisionSetProps {
    die: UseStateHandle<Vec<Decision>>,
}

fn get_valid_dice(dice: String) -> String {
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

fn parse_dice(value: &String) -> DiceSet {
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

#[function_component]
fn DiceTextBox(props: &DiceCallbackProps) -> Html {
    let value_label = use_state(String::default);
    let actual_value = (*value_label).clone();

    let validate = {
        let value_label = value_label.clone();
        let callback = props.callback.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let valid_dice = get_valid_dice(input.value());
            callback.emit(parse_dice(&valid_dice));
            value_label.set(valid_dice);
        })
    };

    return html!(
        <FormControl
            id="input-text"
            ctype={FormControlType::Text}
            class="mb-3"
            placeholder="1d10"
            value={actual_value}
            oninput={validate}
        />
    );
}

#[function_component]
fn DecisionTextBox(props: &DecisionCallbackProps) -> Html {
    // operator: Comparator,
    // decision_dice: DiceSet,
    // decision_value: usize,
    // dice: DiceSet,
    let decision_dice_text = use_state(String::default);
    let decision_dice_value = (*decision_dice_text).clone();

    let dice_text = use_state(String::default);
    let dice_value = (*dice_text).clone();

    let decision_value_text = use_state(|| 0 as usize);
    let decision_value = (*decision_value_text).clone();
    
    let operator_text = use_state(|| Comparison::GreaterEqual.to_string());
    let operator_value = (*operator_text).clone();

    let validate = {
        let callback = props.callback.clone();
        move |dice_value: &String, decision_dice_value: &String, decision_value: &usize, operator_value: &String| {
            let decision_dice = parse_dice(decision_dice_value);
            let dice = parse_dice(dice_value);
            let decision = Decision {
                operator: Comparison::from_str(operator_value).unwrap(),
                decision_dice,
                decision_value: *decision_value,
                dice
            };
            callback.emit(decision);
        }
    };

    let validate_decision = {
        let decision_dice_text = decision_dice_text.clone();
        let validate = validate.clone();
        let dice_value = dice_value.clone();
        let decision_value = decision_value.clone();
        let operator_value = operator_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let valid_dice = get_valid_dice(input.value());
            validate(&dice_value, &valid_dice, &decision_value, &operator_value);
            decision_dice_text.set(valid_dice);
        })
    };

    let validate_dice = {
        let dice_text = dice_text.clone();
        let validate = validate.clone();
        let decision_dice_value = decision_dice_value.clone();
        let decision_value = decision_value.clone();
        let operator_value = operator_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let valid_dice = get_valid_dice(input.value());
            validate(&valid_dice, &decision_dice_value, &decision_value, &operator_value);
            dice_text.set(valid_dice);
        })
    };

    let update_decision_value = {
        let decision_value_text = decision_value_text.clone();
        let validate = validate.clone();
        let decision_dice_value = decision_dice_value.clone();
        let operator_value = operator_value.clone();
        let dice_value = dice_value.clone();
        Callback::from(move |e: InputEvent| {
            let input_elem: HtmlInputElement = e.target_unchecked_into();
            let input = input_elem.value();
            let mut result = 0;
            if input.len() > 0 {
                result = input.parse().unwrap();
            }
            validate(&dice_value, &decision_dice_value, &result, &operator_value);
            decision_value_text.set(result);
        })
    };

    let update_comparison = {
        let operator_text = operator_text.clone();
        let validate = validate.clone();
        let decision_value = decision_value.clone();
        let decision_dice_value = decision_dice_value.clone();
        let dice_value = dice_value.clone();
        Callback::from(move |e: Event| {
            let input_elem: HtmlSelectElement = e.target_unchecked_into();
            let input = input_elem.value();
            validate(&dice_value, &decision_dice_value, &decision_value, &input);
            operator_text.set(input);
        })
    };

    let select_options = {
        let operator_text = operator_text.clone();
        Comparison::iter().enumerate().map(move |(i, v)| {
            return html!(
                <SelectOption key={i} label={v.to_string()} selected={*operator_text == v.to_string()} value={v.to_string()}/>
            );
        }).collect::<Html>()
    };

    return html!(
        <div class="input-group">
            <FormControl
                id="decision_dice"
                ctype={FormControlType::Text}
                class="mb-3"
                placeholder="1d20"
                value={decision_dice_value}
                oninput={validate_decision}
            />
            <FormControl
                id="input-select1"
                ctype={ FormControlType::Select}
                class="mb-3"
                // oninput={update_comparison}
                onchange={update_comparison}
            >
                {select_options}
            </FormControl>
            <FormControl
                id="decision_value"
                ctype={FormControlType::Number { min: Some(0), max: None }}
                class="mb-3"
                placeholder="1d8,1d6"
                value={decision_value.to_string()}
                oninput={update_decision_value}
            />
            <FormControl
                id="dice"
                ctype={FormControlType::Text}
                class="mb-3"
                placeholder="1d8,1d6"
                value={dice_value}
                oninput={validate_dice}
            />
        </div>
    );
}

#[function_component]
fn DicePicker(props: &DecisionSetProps) -> Html {
    let die = props.die.clone();

    let all_dice = die
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let callback = {
                let die = die.clone();

                Callback::from(move |v| {
                    let mut cur = (*die).clone();
                    cur[i] = v;
                    die.set(cur);
                })
            };

            return html!(
                <DecisionTextBox callback={callback}/>
            );
        })
        .collect::<Html>();

    let add_dice = {
        let die = die.clone();
        Callback::from(move |_| {
            let mut new_vec = (*die).clone();
            new_vec.push(Decision::empty());
            die.set(new_vec);
        })
    };

    log!("DicePicker num die", die.len());
    return html!(
        <>
        <div class="input-group mb-3">
            <Button class="btn btn-primary" onclick={add_dice}>{"+"}</Button>
        </div>
        {all_dice}
        </>
    );
}

#[function_component]
fn App() -> Html {
    let hist: UseStateHandle<Vec<f64>> = use_state(|| vec![]);
    let latency = use_state(|| 0.0);
    let dice: UseStateHandle<Vec<Decision>> = use_state(|| vec![]);

    let roll = {
        let hist = hist.clone();
        let latency = latency.clone();
        let dice = dice.clone();
        Callback::from(move |_| {
            let start = Local::now();

            log!("roll num die", (*dice).len());
            hist.set(run_sim(&(*dice)[0], 1000000));
            let end = Local::now();
            let dur = end - start;
            latency.set(dur.num_milliseconds() as f32 / 1000.0);
        })
    };

    let mut xs = Vec::new();
    for i in 0..hist.len() {
        xs.push(i);
    }

    let trace = Scatter::new(xs, (*hist).clone())
        .mode(Mode::LinesMarkersText)
        .name("Scatter");

    let mut plot = Plot::new();
    let template = &*PLOTLY_DARK;
    let layout = Layout::new().template(template);
    plot.set_layout(layout);
    plot.add_trace(trace);

    let mut mean = 0.0;
    for (idx, v) in hist.iter().enumerate() {
        mean += idx as f64 * v;
    }

    log!("App num die", dice.len());
    let temp = dice
        .iter()
        .map(|v| {
            return v.to_string();
        })
        .collect::<Vec<String>>()
        .join("\n");
    log!("app dice", temp);

    return html! {
        <Container size={ContainerSize::Large} fluid={ false }>
            <Row>
                <Column>
                    <h1>{"latency: "}{*latency}</h1>
                    <h1>{"mean: "}{(mean * 100.0).round() / 100.0}</h1>
                    <DicePicker die={dice} />

                    <div class="input-group mb-3">
                        <Button class="btn btn-primary" onclick={roll}>{"Roll!"}</Button>
                    </div>
                    <Plotly plot={plot}/>
                </Column>
            </Row>
        </Container>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
