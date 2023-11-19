import React, { useEffect, useState } from 'react';
import './App.css';
import init, { get_histogram, get_valid_dice } from "dice-lib";
import * as RBS from "react-bootstrap";
import { useHookstate } from '@hookstate/core';
import 'bootstrap/dist/css/bootstrap.css';
import Plot from 'react-plotly.js';

function App() {
  init();

  const dice = useHookstate("");
  const hist = useHookstate([] as number[]);
  document.documentElement.setAttribute("data-bs-theme", "dark");

  return (
    <RBS.Container>
      <RBS.Row>
        <RBS.Form.Group className="mb-3" controlId="exampleRBS.Form.ControlInput1">
          <RBS.Form.Label>dice</RBS.Form.Label>
          <RBS.Form.Control type="text" placeholder="1d4,2d8" value={dice.get()} onChange={(v) => {
            dice.set(get_valid_dice(v.target.value));
          }}/>
          <RBS.Button onClick={v => {
            console.time('doSomething');
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
        layout={ {width: 800, height: 800, title: 'rolls'} }
      />
    </RBS.Container>
  );
}

export default App;
