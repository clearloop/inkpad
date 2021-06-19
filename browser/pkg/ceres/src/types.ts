/**
 * Ceres types
 */
export interface Transaction {
    caller: string;
    address: string;
    balance: number;
    value_transferred: number;
    now: string;
    minimum_balance: string;
}
