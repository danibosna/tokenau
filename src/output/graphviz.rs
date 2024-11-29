use std::fs::File;
use std::io::{self, Write};
use crate::features::transactions::ContractInteraction;

pub fn generate_graphviz_dot(
    interactions: Vec<ContractInteraction>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(output_file)?;
    writeln!(file, "digraph ContractInteractions {{")?;
    writeln!(file, "    rankdir=TB;")?;
    writeln!(file, "    size=\"10,10\";")?;
    writeln!(file, "    node [shape=box, fontsize=10];")?; // Configuración para los nodos
    writeln!(file, "    edge [fontsize=8];")?;            // Configuración para las aristas
    for interaction in interactions {
        let from = interaction.from.to_string();
        let to = interaction.to.to_string();
        let label = interaction.method.to_string();
        writeln!(file, "    \"{}\" -> \"{}\" [label=\"{}\"];", from, to, label)?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

