use crate::{
    contract::{GasMeter, RentParams},
    instantiate, restore,
    schedule::Schedule,
    termination, transfer,
};
use ceres_std::Vec;

/// Extend data
#[derive(Default)]
pub struct Ext {
    pub instantiates: Vec<instantiate::InstantiateEntry>,
    pub restores: Vec<restore::RestoreEntry>,
    pub rent_allowance: [u8; 32],
    pub terminations: Vec<termination::TerminationEntry>,
    pub transfers: Vec<transfer::TransferEntry>,
    pub schedule: Schedule,
    pub rent_params: RentParams,
    pub gas_meter: GasMeter,
    pub contract_deposit: u64,
}
