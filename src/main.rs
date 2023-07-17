use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

use rand::Rng;

use rand::distributions::{Distribution, Uniform};

struct Dice {
    sides: i32,
    distribution: Uniform<i32>,
}

struct DiceSet {
    dice: Vec<Dice>
}

impl Dice {

    fn new(sides: i32) -> Dice {
        let distribution = Uniform::from(1..sides);
        return Dice {sides, distribution};
    }

    fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        return self.distribution.sample(&mut rng);
    }
}

impl DiceSet {

    fn sum(&self) -> i32 {
        let mut value: i32 = 0;
        for die in &self.dice {
            value += die.roll();
        }

        return value;
    }
}

enum DecisionOperator {
    Greater,
    Less,
    GreaterEqual,
    LessEqual
}

struct Decision {
    operator: DecisionOperator,
    dice: DiceSet,
}

#[function_component]
fn App() -> Html {

    let value = use_state(|| 0);
    let dice = DiceSet {dice: Vec::from([Dice::new(10)])};

    let roll = {
        let value = value.clone();
        Callback::from(move |_| value.set(dice.sum()))
    };

    html! {
        <>
            {include_cdn()}
            <Button onclick={roll} text={"Roll!"}/>
            <h1>{*value}</h1>
            {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}