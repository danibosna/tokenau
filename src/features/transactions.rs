use ethers::types::{Log, H160};
use ethers::abi::AbiEncode;

#[derive(Debug)]
pub struct ContractInteraction {
    pub from: String, // Dirección que inicia la interacción
    pub to: String,   // Dirección del contrato objetivo
    pub method: String, // Método llamado
}

pub async fn map_interactions(logs: Vec<Log>) -> Vec<ContractInteraction> {
    logs.into_iter()
        .filter_map(|log| {
            // Filtra eventos específicos, por ejemplo, solo los de transferencia
            let from = log.address.to_string(); // La dirección del contrato que emite el evento
            if let Some(topic) = log.topics.get(1) {
                let to = H160::from_slice(&topic.as_bytes()[12..]);
                // En un caso real, debes obtener el "method" del ABI
                let method = "transfer".to_string(); // Simplificado, normalmente se obtiene del ABI
                return Some(ContractInteraction {
                    from,
                    to: to.to_string(),
                    method,
                });
            }
            None
        })
        .collect()
}

