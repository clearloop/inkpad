//! Contract methods
use crate::{util, Error, Result};
use ceres_std::{String, ToString, Vec};
use ceres_support::types::Metadata;

/// Custom ink method
pub enum InkMethod {
    Deploy,
    Call,
}

impl ToString for InkMethod {
    fn to_string(&self) -> String {
        match &self {
            InkMethod::Call => "call",
            InkMethod::Deploy => "deploy",
        }
        .to_string()
    }
}

impl InkMethod {
    /// Get the input data by method and args
    pub fn parse(&self, metadata: &Metadata, method: &str, args: Vec<Vec<u8>>) -> Result<Vec<u8>> {
        let methods = match self {
            InkMethod::Call => metadata.messages(),
            InkMethod::Deploy => metadata.constructors(),
        };

        // get selector
        let (selector, tys) = methods.get(method).ok_or(Error::GetMethodFailed {
            name: method.to_string(),
        })?;

        // parse arguments
        util::parse_args(selector, args, tys.iter().map(|ty| ty.1).collect())
    }
}
