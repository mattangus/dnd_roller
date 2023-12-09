import { expose } from "comlink";
import {run_sim_decision} from "dice_lib";

const workerExports = {
    run_sim_decision
};
// export type WorkerType = typeof exports;

expose(workerExports);

export default workerExports;