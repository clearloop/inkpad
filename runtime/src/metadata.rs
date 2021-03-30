//! Decoder of *.contract
use alloc::{collections::BTreeMap, string::String, vec::Vec};
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
    pub fn messages(&self) -> BTreeMap<String, (String, Vec<u8>)> {
        let methods: Vec<(String, String, Vec<u8>)> = self
            .spec
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
                    c.args.iter().map(|a| a.r#type.r#type).collect(),
                )
            })
            .collect();

        let mut map = BTreeMap::new();
        for v in methods {
            map.insert(v.0.clone(), (v.1.clone(), v.2));
        }
        map
    }

    /// Get all constructors
    pub fn constructors(&self) -> BTreeMap<String, (String, Vec<u8>)> {
        let methods: Vec<(String, String, Vec<u8>)> = self
            .spec
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
                    c.args.iter().map(|a| a.r#type.r#type).collect(),
                )
            })
            .collect();

        let mut map = BTreeMap::new();
        for v in methods {
            map.insert(v.0.clone(), (v.1.clone(), v.2));
        }
        map
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
    // - - - - - - - - -
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
