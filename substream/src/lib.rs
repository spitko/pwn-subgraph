mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const PWNSIMPLELOAN_TRACKED_CONTRACT: [u8; 20] = hex!("57c88d78f6d08b5c88b4a3b7bbb0c1aa34c3280a");

fn map_pwnsimpleloan_events(blk: &eth::Block, events: &mut contract::Events) {
    events.pwnsimpleloan_loan_claimeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::LoanClaimed::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanLoanClaimed {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            defaulted: event.defaulted,
                            loan_id: event.loan_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pwnsimpleloan_loan_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::LoanCreated::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanLoanCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            factory_address: event.factory_address,
                            factory_data_hash: Vec::from(event.factory_data_hash),
                            loan_id: event.loan_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pwnsimpleloan_loan_expiration_date_extendeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::LoanExpirationDateExtended::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanLoanExpirationDateExtended {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            extended_expiration_date: event.extended_expiration_date.to_u64(),
                            loan_id: event.loan_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pwnsimpleloan_loan_paid_backs.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::LoanPaidBack::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanLoanPaidBack {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            loan_id: event.loan_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.pwnsimpleloan_vault_pulls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::VaultPull::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanVaultPull {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            origin: event.origin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pwnsimpleloan_vault_pushes.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::VaultPush::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanVaultPush {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beneficiary: event.beneficiary,
                        });
                    }

                    None
                })
        })
        .collect());
    events.pwnsimpleloan_vault_push_froms.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PWNSIMPLELOAN_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::pwnsimpleloan_contract::events::VaultPushFrom::match_and_decode(log) {
                        return Some(contract::PwnsimpleloanVaultPushFrom {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beneficiary: event.beneficiary,
                            origin: event.origin,
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_pwnsimpleloan_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.pwnsimpleloan_loan_claimeds.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_claimed", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("defaulted", evt.defaulted)
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_loan_createds.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_created", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("factory_address", Hex(&evt.factory_address).to_string())
            .set("factory_data_hash", Hex(&evt.factory_data_hash).to_string())
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_loan_expiration_date_extendeds.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_expiration_date_extended", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("extended_expiration_date", evt.extended_expiration_date)
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_loan_paid_backs.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_paid_back", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_vault_pulls.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_vault_pull", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("origin", Hex(&evt.origin).to_string());
    });
    events.pwnsimpleloan_vault_pushes.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_vault_push", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beneficiary", Hex(&evt.beneficiary).to_string());
    });
    events.pwnsimpleloan_vault_push_froms.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_vault_push_from", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beneficiary", Hex(&evt.beneficiary).to_string())
            .set("origin", Hex(&evt.origin).to_string());
    });
}


fn graph_pwnsimpleloan_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.pwnsimpleloan_loan_claimeds.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_claimed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("defaulted", evt.defaulted)
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_loan_createds.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_created", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("factory_address", Hex(&evt.factory_address).to_string())
            .set("factory_data_hash", Hex(&evt.factory_data_hash).to_string())
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_loan_expiration_date_extendeds.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_expiration_date_extended", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("extended_expiration_date", evt.extended_expiration_date)
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_loan_paid_backs.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_loan_paid_back", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("loan_id", BigDecimal::from_str(&evt.loan_id).unwrap());
    });
    events.pwnsimpleloan_vault_pulls.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_vault_pull", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("origin", Hex(&evt.origin).to_string());
    });
    events.pwnsimpleloan_vault_pushes.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_vault_push", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beneficiary", Hex(&evt.beneficiary).to_string());
    });
    events.pwnsimpleloan_vault_push_froms.iter().for_each(|evt| {
        tables
            .create_row("pwnsimpleloan_vault_push_from", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beneficiary", Hex(&evt.beneficiary).to_string())
            .set("origin", Hex(&evt.origin).to_string());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_pwnsimpleloan_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_pwnsimpleloan_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_pwnsimpleloan_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
