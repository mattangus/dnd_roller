import React, { ChangeEvent, useLayoutEffect, useRef } from 'react';
import './App.css';
import init, * as dice_lib from "dice_lib";
import * as RBS from "react-bootstrap";
import { useHookstate, State, none } from '@hookstate/core';
import 'bootstrap/dist/css/bootstrap.css';
import Plot from 'react-plotly.js';
import { PlotType } from "plotly.js";
import { TrashFill } from 'react-bootstrap-icons';
// import { wrap, releaseProxy } from "comlink";
// import workerExports from './worker';

// function makeWorkerApiAndCleanup() {
//   // Here we create our worker and wrap it with comlink so we can interact with it
//   const workerUrl = new URL('./worker', import.meta.url);
//   console.log(workerUrl.href);
//   const worker = new Worker(workerUrl,{
//     type: "module"
//   });
//   const workerApi = wrap<typeof workerExports>(worker);

//   // A cleanup function that releases the comlink proxy and terminates the worker
//   const cleanup = () => {
//     workerApi[releaseProxy]();
//     worker.terminate();
//   };

//   const workerApiAndCleanup = { workerApi, cleanup };

//   return workerApiAndCleanup;
// }

interface DecisionText {
  operator: dice_lib.Comparison
  decision_dice: string
  decision_value: number
  dice: string
}

function getEnumKeys<
  T extends string,
  TEnumValue extends string | number,
>(enumVariable: { [key in T]: TEnumValue }) {
  return Object.keys(enumVariable) as Array<T>;
}

function DiceTextBox(props: { dice: State<string> }) {

  const dice = useHookstate(props.dice);

  const validate = (e: React.ChangeEvent<HTMLInputElement>) => {
    const valid = dice_lib.parse_and_discard(e.target.value);
    dice.set(valid);
  };

  return <RBS.Form.Control
    className="mb-3"
    type="text"
    placeholder="1d10"
    value={dice.get()}
    onChange={validate}
  />
}

function DecisionTextBox(props: { decision: State<DecisionText>, deleteCallback: () => void, index?: number}) {
  const decision = useHookstate(props.decision);

  const updateComparison = (e: ChangeEvent<HTMLSelectElement>) => {
    const value = Number(e.target.value);
    decision.operator.set(value);
  };

  const updateDecisionValue = (e: ChangeEvent<HTMLInputElement>) => {
    const value = Number(e.target.value);
    decision.decision_value.set(value);
  };

  const numberBox = props.index !== undefined ? <RBS.Form.Control type="text" className="w-10" value={"#" + props.index} style={{ height: 38 }} readOnly={true}/> : <></>

  // TODO: remove heights from this
  return (
    <div className="input-group">
      {numberBox}
      <DiceTextBox dice={decision.decision_dice}></DiceTextBox>
      <RBS.Form.Select onChange={updateComparison} value={decision.operator.get().toString()} style={{ height: 38 }}>
        {getEnumKeys(dice_lib.Comparison).filter((v) => Number(v) >= 0).map((key, index) => (
          <option key={index} value={key}>
            {dice_lib.Comparison[key]}
          </option>
        ))}
      </RBS.Form.Select>
      <RBS.Form.Control
        type="number"
        className="mb-3"
        value={decision.decision_value.get()}
        onChange={updateDecisionValue}
      />
      <DiceTextBox dice={decision.dice}></DiceTextBox>
      <RBS.Button variant='danger' style={{ height: 38 }} onClick={props.deleteCallback}><TrashFill /></RBS.Button>
    </div >
  );

}

function DndRoller() {
  const defaultDecision = {
    decision_dice: "1d20",
    decision_value: 12,
    operator: dice_lib.Comparison.GreaterThan,
    dice: ""
  } as DecisionText;
  const decisions = useHookstate([structuredClone(defaultDecision)] as DecisionText[]);
  const width = useHookstate(720);
  const hists = useHookstate([] as number[][]);
  document.documentElement.setAttribute("data-bs-theme", "dark");
  const plotContainerRef = useRef<HTMLDivElement | null>(null);

  // const worker = makeWorkerApiAndCleanup();

  const updateSize = () => {
    const current = plotContainerRef.current;
    if (current) {
      const styles = window.getComputedStyle(current);
      const curWidth = parseFloat(styles.width);
      const pad = parseFloat(styles.paddingLeft) + parseFloat(styles.paddingRight);
      width.set(curWidth - pad);
    }
  }
  useLayoutEffect(() => {
    updateSize();
    window.addEventListener("resize", updateSize);
    return () =>
      window.removeEventListener("resize", updateSize);
  });

  const parseAndRun = (dec: DecisionText) => {
    const parsed_decision = new dice_lib.Decision(
      dec.operator,
      dice_lib.parse_dice(dec.decision_dice),
      dec.decision_value,
      dice_lib.parse_dice(dec.dice)
    );
    return dice_lib.run_sim_decision(parsed_decision, 10000000);
    // return worker.workerApi.run_sim_decision(parsed_decision, 10000000);
  };

  const data = hists.map((v, item) => {
    return {
      x: v.get().map((v, i) => i),
      y: Array.from(v.get()),
      type: 'scatter' as PlotType,
      mode: "lines+markers" as "lines+markers",
      name: item.toString()
    }
  })

  return (
    <>
      <RBS.Navbar expand="lg" className="bg-body-tertiary">
        <RBS.Container>
          <RBS.Navbar.Brand>Dnd roller</RBS.Navbar.Brand>
        </RBS.Container>
      </RBS.Navbar>
      <RBS.Container>
        <RBS.Row>
          <RBS.Col>
            <RBS.Form.Group className="mb-3" controlId="exampleRBS.Form.ControlInput1">
              {
                decisions.map((v, i) => {
                  return <DecisionTextBox key={i} decision={v} deleteCallback={() => { decisions[i].set(none) }} index={i} />
                })
              }
            </RBS.Form.Group>
          </RBS.Col>
        </RBS.Row>
        <RBS.Row>
          <RBS.Col>
            <RBS.ButtonGroup className="mb-3 d-flex">
              <RBS.Button onClick={v => {
                decisions.merge([structuredClone(defaultDecision)]);
              }}>Add set</RBS.Button>
              <RBS.Button onClick={v => {
                console.time('run sims');
                // let histValues: number[][] = [];
                // for (let i = 0; i < decisions.length; i++) {
                //   const element = decisions[i].get();

                //   // histValues.push(Array.from(parseAndRun(element)));
                //   let res = parseAndRun(element).then((vals) => Array.from(vals));
                // }
                // const res = Promise.all(decisions.map((v) => parseAndRun(v.get()).then((vals) => Array.from(vals))));
                const res = decisions.map((v) => Array.from(parseAndRun(v.get())));
                hists.set(() => res);
                console.timeEnd('run sims');
              }}>Run</RBS.Button>
            </RBS.ButtonGroup>
          </RBS.Col>
        </RBS.Row>
        <RBS.Row>
          <RBS.Col ref={plotContainerRef}>
            <Plot
              data={data}
              layout={{
                width: width.get(),
                plot_bgcolor: "#dee2e6",
                paper_bgcolor: "#dee2e6",
                title: 'rolls',
                xaxis: { title: "roll value" },
                yaxis: { title: "probability" }
              }}
              config={{
                autosizable: true
              }}
            />
          </RBS.Col>
        </RBS.Row>
        <RBS.Row>
          <RBS.Col>
            <RBS.Table>
              <thead>
                <tr>
                  <th>#</th>
                  <th>Mean value</th>
                </tr>
              </thead>
              <tbody>
                {
                  hists.map((v, i) => {
                    const mean = v.get().reduce((prev, cur, j) => {
                      return prev + cur * j;
                    }, 0);
                    return (
                      <tr key={i}>
                        <td>{i}</td>
                        <td>{mean}</td>
                      </tr>
                    )
                  })
                }
              </tbody>
            </RBS.Table>
          </RBS.Col>
        </RBS.Row>
      </RBS.Container>
    </>
  );
}

function App() {
  let wasm = init();
  console.log("using concurrency %s", navigator.hardwareConcurrency);

  let wasLoaded = useHookstate(() => wasm.then((o) => dice_lib.initThreadPool(navigator.hardwareConcurrency)).then((o) => true));

  if (wasLoaded.promised)
    return <h1>Loading wasm</h1>

  return <DndRoller />
}

export default App;
