//! Europa Runtime
use super::contracts::{Contracts, ContractsEventTypeRegistry};
use sp_runtime::{generic::Header, traits::BlakeTwo256, MultiSignature};
use substrate_subxt::{
    balances::{AccountData, Balances, BalancesEventTypeRegistry},
    extrinsic::DefaultExtra,
    register_default_type_sizes,
    sp_runtime::{MultiAddress, OpaqueExtrinsic},
    system::{System, SystemEventTypeRegistry},
    EventTypeRegistry, NodeTemplateRuntime, Runtime,
};

/// Europa Runtime
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EuropaRuntime;

impl Runtime for EuropaRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_contracts();
        register_default_type_sizes(event_type_registry);
    }
}

impl System for EuropaRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <NodeTemplateRuntime as System>::AccountId;
    type Address = MultiAddress<Self::AccountId, ()>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for EuropaRuntime {
    type Balance = <NodeTemplateRuntime as Balances>::Balance;
}

impl Contracts for EuropaRuntime {}
