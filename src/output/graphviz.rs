use std::fs::File;
use std::io::{self, Write};
use crate::features::transactions::ContractInteraction;

pub fn generate_graphviz_dot(
    interactions: Vec<ContractInteraction>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "digraph ContractInteractions {{")?;
    for interaction in interactions {
        let from = interaction.from.to_string();
        let to = interaction.to.to_string();
        let label = interaction.method.to_string();
        writeln!(file, "    \"{}\" -> \"{}\" [label=\"{}\"];", from, to, label)?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

