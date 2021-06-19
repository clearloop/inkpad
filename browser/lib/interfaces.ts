import * as wasm from "../pkg/ceres_browser";
import { Transaction } from "./types";

/**
 * Ceres runtime
 */
export class Runtime {
    private runtime: wasm.Runtime;

    /**
     * New runtime
     * @param {string} contract - hex of *.contract
     */
    constructor(contract: string) {
        this.runtime = wasm.Runtime.from_contract(contract);
    }

    /**
     * Deploy contract
     *
     * @param {string} method - deploy method
     * @param {string[]} args - passing arguments
     * @param {Transaction} tx - transaction options
     */
    public deploy(method: string, args: string[], tx?: Transaction) {
        this.runtime.deploy(method, JSON.stringify(args), JSON.stringify(tx));
    }

    /**
     * Call contract
     *
     * @param {string} method - deploy method
     * @param {string[]} args - passing arguments
     * @param {Transaction} tx - transaction options
     */
    public call(method: string, args: string[], tx?: Transaction) {
        this.runtime.call(method, JSON.stringify(args), JSON.stringify(tx));
    }

    /**
     * Flush storage
     */
    public flush() {
        this.runtime.flush();
    }
}
