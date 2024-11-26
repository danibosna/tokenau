mod cli;
mod features;
mod integrations;
mod output;

use anyhow::{Context, Result};

use crate::integrations::etherscan::fetch_contract_source;
use crate::output::console::display_contract_details;
use crate::output::graphviz::generate_graphviz_dot;
use crate::features::transactions::map_interactions;
use crate::features::logs::fetch_contract_logs;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    
    println!("Welcome to Smart Contract Auditor!");
    
    let (contract_address, api_key) = cli::input::get_user_input()
        .await
        .context("Failed to get user input")?;    
    // Fetch contract source
    println!("Fetching contract details...");
    let source = fetch_contract_source(&client, &contract_address, &api_key)
        .await
        .context("Failed to fetch contract source")?;
    
    println!("Contract source code fetched successfully!");
    display_contract_details(&source).await;
    
    // Fetch and process logs
    println!("Fetching contract logs...");
    let logs = fetch_contract_logs(&client, &contract_address, &api_key)
        .await
        .context("Failed to fetch contract logs")?;
    
    println!("Logs fetched successfully!");
    let interactions = map_interactions(logs).await;
    
    println!("Displaying contract interactions...");
    for interaction in &interactions {
        println!("{:?}", interaction);
    }
    
    println!("Generating Graphviz output...");
    let _ = generate_graphviz_dot(interactions, "contract_interactions.dot");
    
    Ok(())
}

