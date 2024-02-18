import { expose } from "comlink";
// import {run_sim_decision, Decision} from "dice_lib";


function run_sim_decision_wrapper(decision, iters) {
    console.log("run sim decision wrapper")
    // return run_sim_decision(decision, iters);
    return Float64Array.from([3.0, 3.1]);
}

const workerExports = {
    run_sim_decision_wrapper 
};
// export type WorkerType = typeof exports;

expose(workerExports);

export default workerExports;