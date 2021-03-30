//! Decoder of *.contract
use alloc::{string::String, vec::Vec};
use serde::Deserialize;

/// A struct for operating *.contract
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub source: Source,
    pub contract: Contract,
    pub spec: Spec,
}

impl Metadata {
    /// Get all messages
    pub fn messages(&self) -> Vec<(String, String)> {
        self.spec
            .messages
            .iter()
            .map(|c| {
                (
                    if c.name.len() > 0 {
                        c.name[0].clone()
                    } else {
                        "".into()
                    },
                    c.selector.clone(),
                )
            })
            .collect()
    }

    /// Get all constructors
    pub fn constructors(&self) -> Vec<(String, String)> {
        self.spec
            .constructors
            .iter()
            .map(|c| {
                (
                    if c.name.len() > 0 {
                        c.name[0].clone()
                    } else {
                        "".into()
                    },
                    c.selector.clone(),
                )
            })
            .collect()
    }
}

/// Source section in *.contract
#[derive(Debug, Deserialize)]
pub struct Source {
    /// code hash
    pub hash: String,
    /// hex string in the output metadata, and
    /// this is required in our decoder
    pub wasm: String,
}

/// Contract section in *.contract
#[derive(Debug, Deserialize)]
pub struct Contract {
    /// Just for displaying this on UIs
    pub name: String,
}

/// ABI of methods
#[derive(Debug, Deserialize)]
pub struct Spec {
    pub constructors: Vec<Constructor>,
    pub messages: Vec<Message>,
}

/// Constructor
#[derive(Debug, Deserialize)]
pub struct Constructor {
    pub args: Vec<Arg>,
    pub docs: Vec<String>,
    pub name: Vec<String>,
    pub selector: String,
}

/// Message
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub args: Vec<Arg>,
    pub docs: Vec<String>,
    pub mutates: bool,
    pub name: Vec<String>,
    pub payable: bool,
    // # NOTE
    //
    // For deserializing this field, implement `Deserialize` trait for this.
    //
    // pub return_type: Type,
    pub selector: String,
}

/// Custom arg interface
#[derive(Debug, Deserialize)]
pub struct Arg {
    pub name: String,
    pub r#type: Type,
}

/// Type defination
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub display_name: Vec<String>,
    pub r#type: u8,
}
