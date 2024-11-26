use ethers::types::{Log, H160, H256, Bytes};
use reqwest::Client;
use serde_json::Value;
use ethers::utils::hex;

pub async fn fetch_contract_logs(client: &Client, contract_address: &str, api_key: &str) -> Result<Vec<Log>, reqwest::Error> {
    let url = format!(
        "https://api.etherscan.io/api?module=logs&action=getLogs&address={}&apikey={}",
        contract_address, api_key
    );

    let response = client.get(&url).send().await?.json::<Value>().await?;

    let logs = response["result"]
        .as_array()
        .unwrap_or(&vec![])  // Si no hay resultados, usar un vector vacío
        .iter()
        .map(|log| {
            Log {
                address: log["address"].as_str().unwrap_or_default().parse::<H160>().unwrap_or_default(),
                topics: log["topics"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|t| t.as_str().unwrap_or_default().parse::<H256>().unwrap_or_default())
                    .collect(),
                data: hex::decode(log["data"].as_str().unwrap_or_default()).unwrap_or_default().into(),
                ..Default::default()  // Para los campos que no se pasan explícitamente, se usan los valores por defecto
            }
        })
        .collect();

    Ok(logs)
}

