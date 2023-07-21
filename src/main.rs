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

fn less_than(a: usize, b: usize) -> bool {
    return a < b;
}

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

impl Decision {
    fn empty() -> Decision {
        return Decision {
            operator: less_than,
            decision_dice: DiceSet::empty(),
            decision_value: 0,
            dice: DiceSet::empty(),
        };
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
    log!("validating ", dice.clone());
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
    let caps = re.find(&value);
    match caps {
        None => return DiceSet::empty(),
        Some(x) => {
            log!("found some");
            // let num_dice = x["num_dice"].parse::<usize>().unwrap();
            // let dice_sides = x["num_sides"].parse::<usize>().unwrap();
            // if dice_sides > 1 {
            //     log!("parsing dice set with", num_dice, dice_sides);
            //     return DiceSet {
            //         dice: vec![Dice::new(dice_sides); num_dice],
            //     };
            // } else {
            //     return DiceSet::empty();
            // }
            return DiceSet::empty();
        }
    }
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

    let validate = {
        let temp = decision_dice_value.clone();
        move || {
            log!("called validate");
            let decision = parse_dice(&temp);
            // let dice = parse_dice(&dice_value.clone());
            // props.callback.emit();
        }
    };

    let validate_decision = {
        let decision_dice_text = decision_dice_text.clone();
        let validate = validate.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let valid_dice = get_valid_dice(input.value());
            decision_dice_text.set(valid_dice);
            validate();
        })
    };

    let validate_dice = {
        let dice_text = dice_text.clone();
        let validate = validate.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let valid_dice = get_valid_dice(input.value());
            dice_text.set(valid_dice);
            validate();
        })
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
