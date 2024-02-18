use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
enum State {
    StartState = 1,
    NumDState = 3,
    DState = 2,
    SidesState = 4,
    PlusState = 5,
    OffsetState = 6,
}

type Transition = fn(ch: char) -> (bool, State);

fn start_transition(ch: char) -> (bool, State) {

    match ch {
        ch if ch.is_numeric() => {
            return (true, State::NumDState);
        }
        _ => {
            return (false, State::StartState)
        }
    }
}

fn num_d_transition(ch: char) -> (bool, State) {

    match ch {
        'd' => {
            return (true, State::DState);
        }
        ch if ch.is_numeric() => {
            return (true, State::NumDState);
        }
        _ => {
            return (false, State::NumDState)
        }
    }
}

fn d_transition(ch: char) -> (bool, State) {

    match ch {
        ch if ch.is_numeric() => {
            return (true, State::SidesState);
        }
        _ => {
            return (false, State::DState)
        }
    }
}

fn sides_transition(ch: char) -> (bool, State) {

    match ch {
        ch if ch.is_numeric() => {
            return (true, State::SidesState);
        }
        '+' => {
            return (true, State::PlusState);
        }
        ',' => {
            return (true, State::StartState)
        }
        _ => {
            return (false, State::SidesState)
        }
    }
}

fn plus_transition(ch: char) -> (bool, State) {

    match ch {
        ch if ch.is_numeric() => {
            return (true, State::OffsetState);
        }
        _ => {
            return (false, State::PlusState)
        }
    }
}

fn offset_transition(ch: char) -> (bool, State) {

    match ch {
        ch if ch.is_numeric() => {
            return (true, State::OffsetState);
        }
        ',' => {
            return (true, State::StartState);
        }
        _ => {
            return (false, State::OffsetState)
        }
    }
}


#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_and_discard(dice_str: String) -> String {

    let transitions: HashMap<State, Transition> = HashMap::from([
        (State::StartState, start_transition as Transition),
        (State::NumDState, num_d_transition as Transition),
        (State::DState, d_transition as Transition),
        (State::SidesState, sides_transition as Transition),
        (State::PlusState, plus_transition as Transition),
        (State::OffsetState, offset_transition as Transition),
    ]);

    let mut current = State::StartState; 
    let mut trimmed_string = String::new();

    for ch in dice_str.chars() {
        let (keep, next_state) = transitions.get(&current).unwrap()(ch);
        if keep {
            trimmed_string.push(ch);
        }

        current = next_state;
    }

    return trimmed_string;
}

#[test]
fn test_parse() {
    let regular = ("10d20").to_string();
    let regular_plus = ("1d4+2").to_string();
    let multiple = ("1d4,2d8").to_string();
    let multiple_plus = ("1d3+2,2d8+3").to_string();

    let regular_mangled = ("a1a0ada2a0a").to_string();
    let regular_plus_mangled = ("a1ada4a+a2a").to_string();
    let multiple_mangled = ("a1ada4a,a2ada8a").to_string();
    let multiple_plus_mangled = ("a1ada3a+a2a,a2ada8a+a3a").to_string();

    assert_eq!(parse_and_discard(regular.clone()), regular);
    assert_eq!(parse_and_discard(regular_plus.clone()), regular_plus);
    assert_eq!(parse_and_discard(multiple.clone()), multiple);
    assert_eq!(parse_and_discard(multiple_plus.clone()), multiple_plus);

    assert_eq!(parse_and_discard(regular_mangled.clone()), regular);
    assert_eq!(parse_and_discard(regular_plus_mangled.clone()), regular_plus);
    assert_eq!(parse_and_discard(multiple_mangled.clone()), multiple);
    assert_eq!(parse_and_discard(multiple_plus_mangled.clone()), multiple_plus);
}