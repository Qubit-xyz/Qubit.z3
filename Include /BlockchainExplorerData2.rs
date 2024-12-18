use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

// Placeholder types (since the full implementation of these types is not given in your code).
#[derive(Clone, Debug)]
pub struct TransactionOutput; // Define this struct according to your CryptoTypes
#[derive(Clone, Debug)]
pub struct BaseInput; // Define this struct according to your CryptoTypes
#[derive(Clone, Debug)]
pub struct KeyInput; // Define this struct according to your CryptoTypes
#[derive(Clone, Debug)]
pub struct MultisignatureInput; // Define this struct according to your CryptoTypes
#[derive(Clone, Debug)]
pub struct TransactionOutputReferenceDetails; // Define this struct according to your CryptoTypes
#[derive(Clone, Debug)]
pub struct CryptoPublicKey; // Define this struct according to your CryptoTypes
#[derive(Clone, Debug)]
pub struct BinaryArray(pub Vec<u8>);
#[derive(Clone, Debug)]
pub struct CryptoHash(pub Vec<u8>); // Placeholder for Crypto::Hash type
#[derive(Clone, Debug)]
pub struct CryptoSignature; // Define this struct according to your CryptoTypes

// Rust version of the provided C++ structures

pub struct TransactionOutputDetails {
    pub output: TransactionOutput,
    pub global_index: u64,
}

pub struct BaseInputDetails {
    pub input: BaseInput,
    pub amount: u64,
}

pub struct KeyInputDetails {
    pub input: KeyInput,
    pub mixin: u64,
    pub outputs: Vec<TransactionOutputReferenceDetails>,
}

pub struct MultisignatureInputDetails {
    pub input: MultisignatureInput,
    pub output: TransactionOutputReferenceDetails,
}

// Rust's enum variant for `boost::variant`
pub enum TransactionInputDetails {
    BaseInputDetails(BaseInputDetails),
    KeyInputDetails(KeyInputDetails),
    MultisignatureInputDetails(MultisignatureInputDetails),
}

pub struct TransactionExtraDetails2 {
    pub public_key: CryptoPublicKey,
    pub nonce: BinaryArray,
    pub raw: BinaryArray,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub version: String,
}

pub struct TransactionDetails2 {
    pub hash: CryptoHash,
    pub size: u64,
    pub fee: u64,
    pub total_inputs_amount: u64,
    pub total_outputs_amount: u64,
    pub mixin: u64,
    pub unlock_time: u64,
    pub timestamp: u64,
    pub payment_id: CryptoHash,
    pub has_payment_id: bool,
    pub in_blockchain: bool,
    pub block_hash: CryptoHash,
    pub block_height: u32,
    pub extra: TransactionExtraDetails2,
    pub signatures: Vec<Vec<CryptoSignature>>,
    pub inputs: Vec<TransactionInputDetails>,
    pub outputs: Vec<TransactionOutputDetails>,
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
}

pub struct BlockDetails2 {
    pub major_version: u8,
    pub minor_version: u8,
    pub timestamp: u64,
    pub prev_block_hash: CryptoHash,
    pub nonce: u32,
    pub is_orphaned: bool,
    pub height: u32,
    pub hash: CryptoHash,
    pub difficulty: u64,
    pub reward: u64,
    pub base_reward: u64,
    pub block_size: u64,
    pub transactions_cumulative_size: u64,
    pub already_generated_coins: u64,
    pub already_generated_transactions: u64,
    pub size_median: u64,
    pub penalty: f64,
    pub total_fee_amount: u64,
    pub transactions: Vec<TransactionDetails2>,
}
