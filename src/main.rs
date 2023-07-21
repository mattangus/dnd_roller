use std::vec;

use chrono::prelude::*;
use gloo::console::log;
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use web_sys::{EventTarget, HtmlInputElement};
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
    fn roll(&self) -> usize;
    fn max(&self) -> usize;
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
    fn roll(&self) -> usize {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        return self.distribution.sample(&mut rng);
        // return 0;
    }

    fn max(&self) -> usize {
        return self.sides;
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
    fn roll(&self) -> usize {
        let mut value: usize = 0;
        for die in &self.dice {
            value += die.roll();
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
}

impl DiceSet {
    fn empty() -> DiceSet {
        return DiceSet { dice: vec![] };
    }
}

type Comparator = fn(usize, usize) -> bool;

#[derive(Clone, PartialEq)]
struct Decision {
    operator: Comparator,
    decision_dice: DiceSet,
    decision_value: usize,
    dice: DiceSet,
}

impl Rollable for Decision {
    fn roll(&self) -> usize {
        let decision_roll = self.decision_dice.roll();
        if (self.operator)(decision_roll, self.decision_value) {
            return self.dice.roll();
        }
        return 0;
    }

    fn max(&self) -> usize {
        return self.dice.max();
    }
}

fn run_sim(dice: &dyn Rollable, iters: i32) -> Vec<f64> {
    let mut hist = vec![0.0; dice.max()];
    for _ in 0..iters {
        let roll = dice.roll();
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

fn get_valid_dice(dice: String) -> String {
    let mut ret = String::new();
    let mut has_d = false;
    for char in dice.chars() {
        if char.is_numeric() {
            ret.push(char);
        } else if char == 'd' && !has_d && ret.len() > 0 {
            ret.push(char);
            has_d = true;
        }
    }

    return ret;
}

fn parse_dice(value: &String) -> DiceSet {
    let re = Regex::new(r"(?<num_dice>\d+)d(?<num_sides>\d+)").unwrap();
    let caps = re.captures(&value);
    match caps {
        None => return DiceSet::empty(),
        Some(x) => {
            let num_dice = x["num_dice"].parse::<usize>().unwrap();
            let dice_sides = x["num_sides"].parse::<usize>().unwrap();
            if dice_sides > 1 {
                log!("parsing dice set with", num_dice, dice_sides);
                return DiceSet {
                    dice: vec![Dice::new(dice_sides); num_dice],
                };
            }
            else {
                return DiceSet::empty();
            }
        }
    }
}

#[function_component]
fn DiceTextBox(props: &DiceProps) -> Html {
    let value_label = use_state(String::default);
    let actual_value = (*value_label).clone();

    let validate = {
        let value_label = value_label.clone();
        let dice_set = props.die.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let valid_dice = get_valid_dice(input.value());
            dice_set.set(parse_dice(&valid_dice));
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
fn DicePicker(props: &DiceProps) -> Html {
    return html!(
        // <>
        // <div class="input-group mb-3">
        //     <input type="number" class="form-control text-end" placeholder="Number of Dice" />
        //     <span class="input-group-text" id="basic-addon1">{"d"}</span>
        //     <input type="number" class="form-control" placeholder="Sides of dice" />
        // </div>
        // </>
        <DiceTextBox die={props.die.clone()}/>
    );
}

#[function_component]
fn App() -> Html {
    let hist: UseStateHandle<Vec<f64>> = use_state(|| vec![]);
    let latency = use_state(|| 0.0);
    let value = use_state(String::default);
    let dice: UseStateHandle<DiceSet> = use_state(|| DiceSet::empty());
    // let dice: DiceSet = DiceSet {
    //     dice: Vec::from([Dice::new(10), Dice::new(10), Dice::new(10)]),
    // };
    // let dice = Decision {
    //     operator: |a, b| a >= b,
    //     decision_dice: DiceSet {
    //         dice: vec![Dice::new(20)],
    //     },
    //     decision_value: 10,
    //     dice: DiceSet {
    //         dice: vec![Dice::new(10), Dice::new(10), Dice::new(10)],
    //     },
    // };

    let roll = {
        let hist = hist.clone();
        let latency = latency.clone();
        let value = value.clone();
        let dice = dice.clone();
        Callback::from(move |_| {
            log!("text box value", (*value).clone());
            let start = Local::now();
            hist.set(run_sim(&*dice, 1000000));
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
