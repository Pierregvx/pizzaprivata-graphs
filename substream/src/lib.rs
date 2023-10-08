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


// /// Extracts transactions from the contract
// #[derive(Debug, Deserialize)]
// struct Params {
//     address: String,
// }

const TRACKED_CONTRACTS: [[u8; 20]; 2] = [
    hex!("87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2"),
    hex!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D")
];


#[substreams::handlers::map]
fn map_transactions(
    // params: String,
    blk: eth::Block) -> Result<tx::EthTransactions, substreams::errors::Error> {
    // let query: Params = serde_qs::from_str(params.as_str()).unwrap();
        let ADDRESS : [u8; 20] = hex!("79203Ead972B87409D96C8B6BE8aC0681887B4A0");

    Ok(tx::EthTransactions {
        transactions: blk
            .transactions()
            .filter(|transaction| {
                let to_vec = transaction.to.clone();  // Clone the vector
                let to_array: Result<[u8; 20], _> = to_vec.try_into();  // Convert Vec<u8> into [u8; 20]
            
                match to_array {
                    Ok(arr) => transaction.from == ADDRESS && TRACKED_CONTRACTS.contains(&arr),
                    Err(_) => false,  // If the conversion fails, just return false for the filter
                }
            })
            
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