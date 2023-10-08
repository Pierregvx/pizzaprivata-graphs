use substreams::proto;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use hex::encode as hex_encode;

use crate::pb::eth::tx::v1::EthTransactions;

pub fn transactions_to_kv_ops(transactions: EthTransactions) -> Result<KvOperations, substreams::errors::Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();

    for (ordinal, transaction) in transactions.transactions.iter().enumerate() {
        let val = proto::encode(transaction).unwrap();
        let key = hex_encode(&transaction.trx_hash); // Convert Vec<u8> to hexadecimal string
    
        kv_ops.push_new(key, val, ordinal as u64);
    }

    Ok(kv_ops)
}
