//! Contract Module
use codec::{Decode, Encode};
use substrate_subxt::{balances::Balances, system::System, Call, Event};

/// Gas units
pub type Gas = u64;

#[substrate_subxt::module]
pub trait Contracts: System + Balances {}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct InstantiateWithCodeCall<'a, T: Contracts> {
    /// Initial balance transferred to the contract.
    #[codec(compact)]
    pub endowment: <T as Balances>::Balance,
    /// Gas limit.
    #[codec(compact)]
    pub gas_limit: Gas,
    /// Wasm code
    pub code: &'a [u8],
    /// Data to initialize the contract with.
    pub data: &'a [u8],
    /// contract salt
    pub salt: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct CallCall<'a, T: Contracts> {
    /// Address of the contract.
    pub dest: &'a <T as System>::Address,
    /// Value to transfer to the contract.
    #[codec(compact)]
    pub value: <T as Balances>::Balance,
    /// Gas limit.
    #[codec(compact)]
    pub gas_limit: Gas,
    /// Data to send to the contract.
    pub data: &'a [u8],
}

/// Instantiated event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct InstantiatedEvent<T: Contracts> {
    /// Caller that instantiated the contract.
    pub deployer: <T as System>::AccountId,
    /// The address of the contract.
    pub contract: <T as System>::AccountId,
}

/// Evicted
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Evicted<T: Contracts> {
    pub contract: <T as System>::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Terminated<T: Contracts> {
    /// Caller that instantiated the contract.
    pub caller: <T as System>::AccountId,
    /// The address of the contract.
    pub contract: <T as System>::AccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct Restored<T: Contracts> {
    /// Caller that instantiated the contract.
    pub caller: <T as System>::AccountId,
    /// The address of the contract.
    pub contract: <T as System>::AccountId,
    /// Hash
    pub hash: <T as System>::Hash,
    /// Balance
    pub value: <T as Balances>::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CodeStored<T: Contracts> {
    /// Hash
    pub hash: <T as System>::Hash,
}

#[derive(Clone, Debug, PartialEq, Event, Decode)]
pub struct ScheduleUpdated<T: Contracts> {
    pub update: u32,
    pub _marker: std::marker::PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Event, Decode)]
pub struct ContractEmitted<T: Contracts> {
    /// Caller that instantiated the contract.
    pub caller: <T as System>::AccountId,
    pub data: Vec<u8>,
}

pub struct CodeRemoved<T: Contracts> {
    pub hash: <T as System>::Hash,
}
