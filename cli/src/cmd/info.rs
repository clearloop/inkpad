//! List all contracts
use crate::Result;
use ceres_runtime::Runtime;

/// List all contracts
pub fn exec(rt: &Runtime) -> Result<()> {
    let meta = rt.metadata.clone();
    let mut output = String::new();
    output.push_str(&format!("\n\tname: {}\n", meta.contract.name));
    output.push_str(&format!("\tcode_hash: {}\n", meta.source.hash));
    output.push_str(&format!("\tcontructors:\n"));
    meta.constructors().iter().for_each(|(k, v)| {
        output.push_str(&format!(
            "\t\t - {} [ {} ]\n",
            k,
            v.1.iter()
                .filter_map(|ty| ty.0.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    });
    output.push_str(&format!("\tmethods:\n"));
    meta.messages().iter().for_each(|(k, v)| {
        output.push_str(&format!(
            "\t\t - {} [ {} ]\n",
            k,
            v.1.iter()
                .filter_map(|ty| ty.0.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    });

    println!("{}", output);
    Ok(())
}
