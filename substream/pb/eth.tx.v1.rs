// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTransactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<EthTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTransaction {
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
    /// string gas_price = 2;
    #[prost(string, tag="3")]
    pub gas_limit: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    /// string value = 5;
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub s: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub trx_hash: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
