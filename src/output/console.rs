use crate::integrations::etherscan::ContractSource;

/// Muestra detalles de un contrato inteligente en la consola
pub async fn display_contract_details(contract_source: &ContractSource) {
    println!("Detalles del Contrato Inteligente:");
    println!("----------------------------------");
    println!("Nombre del Contrato: {}", contract_source.result[0].ContractName);
    println!("Versión del Compilador: {}", contract_source.result[0].CompilerVersion);
    println!("Código Fuente:");
    println!("{}", contract_source.result[0].SourceCode);

    match &contract_source.result[0].ABI {
        Some(abi) => {
            // Aquí `abi` es un `&String`
            println!("ABI encontrado: {}", abi);
        }
        None => {
            println!("ABI no encontrado");
        }
    }

    println!("----------------------------------");
}

