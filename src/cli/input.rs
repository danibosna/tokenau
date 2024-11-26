use dialoguer::{Input, Password};
use std::error::Error;
use anyhow::Result;

pub async fn get_user_input() -> Result<(String, String)> {
    let contract_address: String = Input::new()
        .with_prompt("Enter the contract address")
        .interact_text()?;
    let api_key: String = Password::new()
        .with_prompt("Enter your Etherscan API Key")
        .interact()?;
    Ok((contract_address, api_key))
}

