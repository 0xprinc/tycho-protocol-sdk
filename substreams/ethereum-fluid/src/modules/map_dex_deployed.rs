use std::str::FromStr;

use ethabi::ethereum_types::Address;
use serde::Deserialize;
use substreams::prelude::BigInt;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::{event_handler::EventHandler, hex::Hexable};

use crate::abi::factory::events::LogDexDeployed;

use tycho_substreams::prelude::*;

#[derive(Debug, Deserialize)]
struct Params {
    factory_address: String,
    protocol_type_name: String,
}

#[substreams::handlers::map]
pub fn map_dex_deployed(
    params: String,
    block: eth::Block,
) -> Result<BlockChanges, substreams::errors::Error> {
    let mut new_dexes: Vec<TransactionChanges> = vec![];

    let params: Params = serde_qs::from_str(params.as_str()).expect("Unable to deserialize params");

    get_dexes(&block, &mut new_dexes, &params);

    let tycho_block: Block = (&block).into();

    Ok(BlockChanges { block: Some(tycho_block), changes: new_dexes })
}

fn get_dexes(block: &eth::Block, new_dexes: &mut Vec<TransactionChanges>, params: &Params) {
    // Extract new dexes from LogDexDeployed events
    let mut on_dex_deployed = |event: LogDexDeployed, _tx: &eth::TransactionTrace, _log: &eth::Log| {
        let tycho_tx: Transaction = _tx.into();

        new_dexes.push(TransactionChanges {
            tx: Some(tycho_tx.clone()),
            contract_changes: vec![],
            entity_changes: vec![],
            component_changes: vec![ProtocolComponent {
                id: event.dex.to_hex(),
                tokens: vec![], // Add relevant tokens if any
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
                    name: params.protocol_type_name.to_string(),
                    financial_type: FinancialType::Swap.into(),
                    attribute_schema: vec![],
                    implementation_type: ImplementationType::Custom.into(),
                }),
                tx: Some(tycho_tx),
            }],
            balance_changes: vec![],
        })
    };

    let mut eh = EventHandler::new(block);

    eh.filter_by_address(vec![Address::from_str(&params.factory_address).unwrap()]);

    eh.on::<LogDexDeployed, _>(&mut on_dex_deployed);
    eh.handle_events();
}