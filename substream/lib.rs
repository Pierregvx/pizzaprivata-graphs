mod pb;
#[path = "kv_out.rs"]
mod kv;
use substreams::errors::Error;
use hex_literal::hex;
use pb::eth::tx::v1 as tx;
use substreams::prelude::*;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2::{self as eth, Block};
use substreams_entity_change::tables::Tables;
substreams_ethereum::init!();
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use hex::encode as hex_encode;
use substreams::proto;
use crate::pb::eth::tx::v1::EthTransactions;


/// Extracts transactions from the contract

const TARGET_DATA: &[u8] = &hex!("24856bc30000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000020b080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000007170f2c279bf0b600000000000000000000000000000000000000000000000000000000000001000000000000000000000000002159437e85f64f7ee1017129aeca80481f1af1a200000000000000000000000000000000000000000000000007170f2c279bf0b600000000000000000000000000000000000000000065f82b7ea37717a0f8bb3900000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000ab84f9a1ad03ce87447eb75f31a3e803d16fcc92"); // replace with your full hex string

#[substreams::handlers::map]
fn map_transactions(blk: eth::Block) -> Result<tx::EthTransactions, substreams::errors::Error> {
    Ok(tx::EthTransactions {
        transactions: blk
            .transactions()
            .filter(|transaction| transaction.input == TARGET_DATA)
            .map(|transaction| {
                substreams::log::info!("Transaction seen");

                tx::EthTransaction {
                    trx_hash: transaction.hash.clone(),
                    nonce: transaction.nonce.to_string(),
                    gas_limit: transaction.gas_limit.to_string(),
                    data: transaction.input.clone(),
                    // Assuming the transaction is already signed, you might not need these
                    v: transaction.v.clone(),
                    to: transaction.to.to_ascii_lowercase(),
                    r: transaction.r.clone(),
                    s: transaction.s.clone(),
                }
            })
            .collect(),
    })
}



/// Store the transactions for the specific TRACKED_CONTRACT
#[substreams::handlers::store]
fn store_transactions(transactions: tx::EthTransactions, s: StoreAddInt64) {
    log::info!("Transaction state builder");
    for transaction in transactions.transactions {
        log::info!("Found a transaction {}", Hex(&transaction.trx_hash));
        // Add your storing logic here
    }
}


#[substreams::handlers::map]
pub fn graph_out(transactions: tx::EthTransactions) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for transaction in transactions.transactions.into_iter() {
        tables
            .create_row("Transaction", &hex::encode(&transaction.trx_hash))
            .set("nonce", transaction.nonce)
            // .set("gas_price", transaction.gas_price)
            .set("gasLimit", transaction.gas_limit)
            .set("to", "transaction.to.to_ascii_lowercase()")
            // .set("value", transaction.value)
            .set("data", "transaction.data.to_ascii_lowercase()")
            // .set("v", transaction.v)
            // .set("r", transaction.r)
            // .set("s", transaction.s)
            ;
    }

    Ok(tables.to_entity_changes())
}

pub fn kv_out(
    transactions: EthTransactions,
) -> Result<KvOperations, Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    for (ordinal, transaction) in transactions.transactions.iter().enumerate() {
        let val = proto::encode(transaction).unwrap();
        let key = hex_encode(&transaction.trx_hash); // Convert Vec<u8> to hexadecimal string
        kv_ops.push_new(key, val, ordinal as u64);
    }

    Ok(kv_ops)
}