use std::collections::HashMap;
use substreams::store::{StoreGet, StoreGetProto};
use substreams_ethereum::pb::eth::v2::{self as eth};
use tycho_substreams::contract::extract_contract_changes;
use tycho_substreams::prelude::*;

#[substreams::handlers::map]
pub fn map_protocol_changes(
    block: eth::Block,
    store: StoreGetProto<ProtocolComponent>,
) -> Result<BlockChanges, substreams::errors::Error> {
    let mut transaction_contract_changes: HashMap<_, TransactionChanges> = HashMap::new();

    extract_contract_changes(
        &block,
        |addr| {
            store
                .get_last(format!("dex:{0}", hex::encode(addr)))
                .is_some()
        },
        &mut transaction_contract_changes,
    );

    let tycho_block: Block = (&block).into();

    Ok(BlockChanges {
        block: Some(tycho_block),
        changes: vec![],
    })
}
