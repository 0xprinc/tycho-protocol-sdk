use std::str::FromStr;
use ethabi::ethereum_types::Address;
use serde::Deserialize;
use substreams::prelude::*;
use substreams_ethereum::pb::eth::v2::{self as eth};
use tycho_substreams::contract::extract_contract_changes;
use tycho_substreams::prelude::*;
use substreams::log::info;
use std::collections::HashMap;
use anyhow::Error;

use crate::abi::factory::events::{
    LogDexDeployed,
    LogSetDeployer,
    LogSetGlobalAuth,
    LogSetDexAuth,
    LogSetDexDeploymentLogic,
};

#[derive(Debug, Deserialize)]
struct Params {
    factory_address: String,
    protocol_type_name: String,
}

// #[substreams::handlers::map]
pub fn map_dex_deployed(
    params: String,
    block: eth::Block,
// ) -> Result<BlockChanges, substreams::errors::Error> {
) -> Result<(), substreams::errors::Error> {
    // let mut new_changes: Vec<TransactionChanges> = vec![];

    // let params: Params = serde_qs::from_str(&params).map_err(|_| {
    //     Error::msg("Unable to deserialize params".to_string())
    // })?;

    // let mut transaction_contract_changes: HashMap<_, TransactionChanges> = HashMap::new();

    // extract_contract_changes(
    //     &block,
    //     |addr| {
    //         // Convert addr (&[u8]) to Address
    //         let addr_as_address = Address::from_slice(addr);
            
    //         // Compare with the factory address
    //         addr_as_address == Address::from_str(&params.factory_address).unwrap()
    //     },
    //     &mut transaction_contract_changes,
    // );

    // for (_tx_hash, tx_changes) in transaction_contract_changes {
    //     for log in tx_changes.logs {
    //         match log.event {
    //             "LogDexDeployed" => {
    //                 let event: LogDexDeployed = log.decode()?;
    //                 handle_log_dex_deployed(event, &mut new_changes, &params)?;
    //             },
    //             "LogSetDeployer" => {
    //                 let event: LogSetDeployer = log.decode()?;
    //                 handle_log_set_deployer(event, &mut new_changes, &params)?;
    //             },
    //             "LogSetGlobalAuth" => {
    //                 let event: LogSetGlobalAuth = log.decode()?;
    //                 handle_log_set_global_auth(event, &mut new_changes, &params)?;
    //             },
    //             "LogSetDexAuth" => {
    //                 let event: LogSetDexAuth = log.decode()?;
    //                 handle_log_set_dex_auth(event, &mut new_changes, &params)?;
    //             },
    //             "LogSetDexDeploymentLogic" => {
    //                 let event: LogSetDexDeploymentLogic = log.decode()?;
    //                 handle_log_set_dex_deployment_logic(event, &mut new_changes, &params)?;
    //             },
    //             _ => {}
    //         }
    //     }
    // }

    // let tycho_block: Block = (&block).into();

    // Ok(BlockChanges {
    //     block: Some(tycho_block),
    //     changes: new_changes,
    // })

    Ok({})
}

fn handle_log_dex_deployed(
    event: LogDexDeployed,
    changes: &mut Vec<TransactionChanges>,
    params: &Params,
) -> Result<(), substreams::errors::Error> {
    info!("Dex deployed: {:?}", event.dex);
    let protocol_component = ProtocolComponent {
        id: String::from_utf8_lossy(&event.dex).to_string(),
        tokens: vec![],
        contracts: vec![],
        static_att: vec![
            Attribute {
                name: "dex_id".to_string(),
                value: BigInt::from(event.dex_id).to_signed_bytes_be(),
                change: ChangeType::Creation.into(),
            },
            Attribute {
                name: "dex_address".to_string(),
                value: event.dex.clone(),
                change: ChangeType::Creation.into(),
            },
        ],
        change: i32::from(ChangeType::Creation),
        protocol_type: Some(ProtocolType {
            name: params.protocol_type_name.clone(),
            financial_type: FinancialType::Swap.into(),
            attribute_schema: vec![],
            implementation_type: ImplementationType::Custom.into(),
        }),
        tx: None,
    };

    changes.push(TransactionChanges {
        tx: None,
        contract_changes: vec![],
        entity_changes: vec![],
        component_changes: vec![protocol_component],
        balance_changes: vec![],
    });

    Ok(())
}

fn handle_log_set_deployer(
    event: LogSetDeployer,
    changes: &mut Vec<TransactionChanges>,
    params: &Params,
) -> Result<(), substreams::errors::Error> {
    info!("Set Deployer: {:?}, Allowed: {}", event.deployer, event.allowed);
    // Update deployer status logic here
    // Example: Create or update a ProtocolComponent for the deployer

    let protocol_component = ProtocolComponent {
        id: format!("deployer:{}", String::from_utf8_lossy(&event.deployer)),
        tokens: vec![],
        contracts: vec![],
        static_att: vec![
            Attribute {
                name: "allowed".to_string(),
                value: if event.allowed { "true" } else { "false" }.as_bytes().to_vec(),
                change: ChangeType::Update.into(),
            },
        ],
        change: i32::from(ChangeType::Update),
        protocol_type: None,
        tx: None,
    };

    changes.push(TransactionChanges {
        tx: None,
        contract_changes: vec![],
        entity_changes: vec![],
        component_changes: vec![protocol_component],
        balance_changes: vec![],
    });

    Ok(())
}

fn handle_log_set_global_auth(
    event: LogSetGlobalAuth,
    changes: &mut Vec<TransactionChanges>,
    params: &Params,
) -> Result<(), substreams::errors::Error> {
    info!("Set GlobalAuth: {:?}, Allowed: {}", event.global_auth, event.allowed);
    // Update globalAuth status logic here

    let protocol_component = ProtocolComponent {
        id: format!("globalAuth:{}", String::from_utf8_lossy(&event.global_auth)),
        tokens: vec![],
        contracts: vec![],
        static_att: vec![
            Attribute {
                name: "allowed".to_string(),
                value: if event.allowed { "true" } else { "false" }.as_bytes().to_vec(),
                change: ChangeType::Update.into(),
            },
        ],
        change: i32::from(ChangeType::Update),
        protocol_type: None,
        tx: None,
    };

    changes.push(TransactionChanges {
        tx: None,
        contract_changes: vec![],
        entity_changes: vec![],
        component_changes: vec![protocol_component],
        balance_changes: vec![],
    });

    Ok(())
}

fn handle_log_set_dex_auth(
    event: LogSetDexAuth,
    changes: &mut Vec<TransactionChanges>,
    params: &Params,
) -> Result<(), substreams::errors::Error> {
    info!("Set DexAuth: {:?}, Allowed: {}, Dex: {:?}", event.dex_auth, event.allowed, event.dex);
    // Update dexAuth status logic here

    let protocol_component = ProtocolComponent {
        id: format!("dexAuth:{}:{}", String::from_utf8_lossy(&event.dex_auth), String::from_utf8_lossy(&event.dex)),
        tokens: vec![],
        contracts: vec![],
        static_att: vec![
            Attribute {
                name: "allowed".to_string(),
                value: if event.allowed { "true" } else { "false" }.as_bytes().to_vec(),
                change: ChangeType::Update.into(),
            },
        ],
        change: i32::from(ChangeType::Update),
        protocol_type: None,
        tx: None,
    };

    changes.push(TransactionChanges {
        tx: None,
        contract_changes: vec![],
        entity_changes: vec![],
        component_changes: vec![protocol_component],
        balance_changes: vec![],
    });

    Ok(())
}

fn handle_log_set_dex_deployment_logic(
    event: LogSetDexDeploymentLogic,
    changes: &mut Vec<TransactionChanges>,
    params: &Params,
) -> Result<(), substreams::errors::Error> {
    info!("Set DexDeploymentLogic: {:?}, Allowed: {}", event.dex_deployment_logic, event.allowed);
    // Update dexDeploymentLogic status logic here

    let protocol_component = ProtocolComponent {
        id: format!("dexDeploymentLogic:{}", String::from_utf8_lossy(&event.dex_deployment_logic)),
        tokens: vec![],
        contracts: vec![],
        static_att: vec![
            Attribute {
                name: "allowed".to_string(),
                value: if event.allowed { "true" } else { "false" }.as_bytes().to_vec(),
                change: ChangeType::Update.into(),
            },
        ],
        change: i32::from(ChangeType::Update),
        protocol_type: None,
        tx: None,
    };

    changes.push(TransactionChanges {
        tx: None,
        contract_changes: vec![],
        entity_changes: vec![],
        component_changes: vec![protocol_component],
        balance_changes: vec![],
    });

    Ok(())
}
