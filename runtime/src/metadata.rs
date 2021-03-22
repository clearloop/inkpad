use serde::Deserialize;

/// A struct for operating *.contract
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub source: Source,
    pub contract: Contract,
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

/// Constructor
#[derive(Debug, Deserialize)]
pub struct Constructor {
    pub args: Vec<Arg>,
    pub docs: Vec<String>,
    pub name: Vec<String>,
    pub selector: String,
}

/// ABI of methods
#[derive(Deserialize)]
pub struct Spec {
    pub constructors: Vec<Constructor>,
}

/// Type defination
#[derive(Debug, Deserialize)]
pub struct Type {
    #[serde(rename(deserialize = "camelCase"))]
    pub display_name: Vec<String>,
    pub r#type: u8,
}

/// Custom arg interface
#[derive(Debug, Deserialize)]
pub struct Arg {
    pub name: String,
    pub r#type: Type,
}
