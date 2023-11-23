import React, { ChangeEvent, ChangeEventHandler, ReactEventHandler } from 'react';
import './App.css';
import init, * as dice_lib from "dice_lib";
import * as RBS from "react-bootstrap";
import { useHookstate, State } from '@hookstate/core';
import 'bootstrap/dist/css/bootstrap.css';
import Plot from 'react-plotly.js';

function getEnumKeys<
    T extends string,
    TEnumValue extends string | number,
>(enumVariable: { [key in T]: TEnumValue }) {
    return Object.keys(enumVariable) as Array<T>;
}

function DiceTextBox(props: { dice: State<dice_lib.DiceSet> }) {

  const dice = useHookstate(props.dice);

  const validate = (e: React.ChangeEvent<HTMLInputElement>) => {
    const valid = dice_lib.get_valid_dice(e.target.value);
    const parsed = dice_lib.DiceSet.from_string(valid);
    dice.set(parsed);
  };

  return <RBS.Form.Control
    className="mb-3"
    type="text"
    placeholder="1d10"
    value={dice.get().to_str()}
    onChange={validate}
  />
}

function DecisionTextBox(props: { decision: State<dice_lib.Decision> }) {
  const decision = useHookstate(props.decision);

  const updateComparison = (e: ChangeEvent<HTMLSelectElement>) => {
    const value: string = e.target.value
    decision.operator.set(value as any);
  };

  const updateDecisionValue = (e: ChangeEvent<HTMLInputElement>) => {
    const value = Number(e.target.value);
    decision.decision_value.set(value);
  };


  return (
  <div className="input-group">
    <DiceTextBox dice={decision.decision_dice}></DiceTextBox>
    <RBS.Form.Select onChange={updateComparison} value={decision.operator.get()}>
    {getEnumKeys(dice_lib.Comparison).map((key, index) => (
        <option key={index} value={dice_lib.Comparison[key]}>
          {dice_lib.Comparison[key]}
        </option>
      ))}
    </RBS.Form.Select>
    <RBS.Form.Control
      type="number"
      className="mb-3"
      value={ decision.decision_value.get() }
      onChange={ updateDecisionValue }
    />
    <DiceTextBox dice={decision.dice}></DiceTextBox>
  </div > 
  );

}

function App() {
  init();

  const dice = useHookstate("");
  const decision = useHookstate(new dice_lib.Decision(
    dice_lib.Comparison.GreaterEqual,
    dice_lib.DiceSet.empty(),
    12,
    dice_lib.DiceSet.empty()
  ));
  //   decisionDiceText: "1d20",
  //   decisionValue: 12,
  //   operator: dice_lib.Comparison.GreaterThan,
  //   diceText: ""
  // } as Decision)
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
            hist.set(Array.from(dice_lib.get_histogram(dice.get(), 1000000)));
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
