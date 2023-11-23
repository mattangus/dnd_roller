import React, { ChangeEvent, ChangeEventHandler, ReactEventHandler } from 'react';
import './App.css';
import init, { get_histogram, get_valid_dice } from "dice-lib";
import * as RBS from "react-bootstrap";
import { useHookstate, State } from '@hookstate/core';
import 'bootstrap/dist/css/bootstrap.css';
import Plot from 'react-plotly.js';

enum Comparison {
  LessThan = "<",
  GreaterThan = ">",
  LessEqual = "<=",
  GreaterEqual = ">=",
  Equal = "=",
}

interface Decision {
  decisionDiceText: string;
  diceText: string;
  decisionValue: number;
  operator: Comparison;
}

function getEnumKeys<
    T extends string,
    TEnumValue extends string | number,
>(enumVariable: { [key in T]: TEnumValue }) {
    return Object.keys(enumVariable) as Array<T>;
}

function DiceTextBox(props: { diceText: State<string> }) {

  const diceText = useHookstate(props.diceText);

  const validate = (e: React.ChangeEvent<HTMLInputElement>) => {
    diceText.set(get_valid_dice(e.target.value))
  };

  return <RBS.Form.Control
    className="mb-3"
    type="text"
    placeholder="1d10"
    value={diceText.get()}
    onChange={validate}
  />
}

function DecisionTextBox(props: { decision: State<Decision> }) {
  const decision = useHookstate(props.decision);

  const updateComparison = (e: ChangeEvent<HTMLSelectElement>) => {
    const value: string = e.target.value
    decision.operator.set(value as any);
  };

  return (
  <div className="input-group">
    <DiceTextBox diceText={decision.decisionDiceText}></DiceTextBox>
    <RBS.Form.Select onChange={updateComparison} value={decision.operator.get()}>
    {getEnumKeys(Comparison).map((key, index) => (
        <option key={index} value={Comparison[key]}>
          {Comparison[key]}
        </option>
      ))}
    </RBS.Form.Select>
    {/* <RBS.Form.Control
        ctype={ RBS.Form.ControlType::Select}
        class="mb-3"
        // oninput={update_comparison}
      onchange={update_comparison}
    >
      {select_options}
    </RBS.Form.Control>
    <RBS.Form.Control
      id="decision_value"
      ctype={RBS.Form.ControlType:: Number { min: Some(0), max: None
}}
class="mb-3"
placeholder = "1d8,1d6"
value = { decision_value.to_string() }
oninput = { update_decision_value }
  />
  <RBS.Form.Control
    ctype={RBS.Form.ControlType:: Text}
    class="mb-3"
    placeholder="1d8,1d6"
    value={dice_value}
    oninput={validate_dice}
  />*/}
  </div > 
  );

}

function App() {
  init();

  const dice = useHookstate("");
  const decision = useHookstate({
    decisionDiceText: "",
    decisionValue: 0,
    operator: Comparison.GreaterThan,
    diceText: ""
  } as Decision)
  const hist = useHookstate([] as number[]);
  document.documentElement.setAttribute("data-bs-theme", "dark");

  return (
    <RBS.Container>
      <RBS.Row>
        <RBS.Form.Group className="mb-3" controlId="exampleRBS.Form.ControlInput1">
          <RBS.Form.Label>dice</RBS.Form.Label>
          <DecisionTextBox decision={decision} />
          <RBS.Button onClick={v => {
            console.time('doSomething');
            console.log(dice)
            hist.set(Array.from(get_histogram(dice.get(), 1000000)));
            console.timeEnd('doSomething');
          }}>Run</RBS.Button>
        </RBS.Form.Group>
      </RBS.Row>
      <Plot
        data={[
          {
            x: hist.get().map((v, i) => i),
            y: Array.from(hist.get()),
            type: 'scatter',
            mode: "lines+markers"
          }
        ]}
        layout={{ width: 800, height: 800, title: 'rolls' }}
      />
    </RBS.Container>
  );
}

export default App;
