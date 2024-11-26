use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct ContractSource {
    pub status: String,
    pub message: String,
    pub result: Vec<SourceResult>,
}

#[derive(Deserialize, Debug)]
pub struct SourceResult {
    pub SourceCode: String,
    pub ABI: Option<String>,
    pub ContractName: String,
    pub CompilerVersion: String,
    pub OptimizationUsed: String,
    pub Runs: String,
}

pub async fn fetch_contract_source(
    client: &Client,
    contract_address: &str,
    api_key: &str,
) -> Result<ContractSource> {
    let url = format!(
        "https://api.etherscan.io/api?module=contract&action=getsourcecode&address={}&apikey={}",
        contract_address, api_key
    );
    let response = client.get(&url).send().await?.json::<ContractSource>().await?;
    Ok(response)
}

