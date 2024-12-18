use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionRemoveReason {
    INCLUDED_IN_BLOCK = 0,
    TIMEOUT = 1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputToKeyDetails {
    pub tx_out_key: CryptoPublicKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputMultisignatureDetails {
    pub keys: Vec<CryptoPublicKey>,
    pub required_signatures: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TransactionOutputDetails {
    ToKey(TransactionOutputToKeyDetails),
    Multisignature(TransactionOutputMultisignatureDetails),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputReferenceDetails {
    pub transaction_hash: CryptoHash,
    pub number: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInputGenerateDetails {
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInputToKeyDetails {
    pub output_indexes: Vec<u32>,
    pub key_image: CryptoKeyImage,
    pub mixin: u64,
    pub outputs: Vec<TransactionOutputReferenceDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionInputMultisignatureDetails {
    pub signatures: u32,
    pub output: TransactionOutputReferenceDetails,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TransactionInputDetails {
    Generate(TransactionInputGenerateDetails),
    ToKey(TransactionInputToKeyDetails),
    Multisignature(TransactionInputMultisignatureDetails),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionExtraDetails {
    pub padding: Vec<usize>,
    pub public_key: Vec<CryptoPublicKey>,
    pub nonce: Vec<String>,
    pub raw: Vec<u8>,
    // additional fields:
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionOutputDetails2 {
    pub output: TransactionOutput,
    pub global_index: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BaseInputDetails {
    pub input: BaseInput,
    pub amount: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyInputDetails {
    pub input: KeyInput,
    pub mixin: u64,
    pub outputs: Vec<TransactionOutputReferenceDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultisignatureInputDetails {
    pub input: MultisignatureInput,
    pub output: TransactionOutputReferenceDetails,
}

pub type TransactionInputDetails2 = TransactionInputDetails;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionExtraDetails2 {
    pub padding: Vec<usize>,
    pub public_key: CryptoPublicKey,
    pub nonce: BinaryArray,
    pub raw: BinaryArray,
    // non-privacy fields:
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub tx_key: CryptoSecretKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionDetails {
    pub hash: CryptoHash,
    pub size: u64,
    pub fee: u64,
    pub total_inputs_amount: u64,
    pub total_outputs_amount: u64,
    pub mixin: u64,
    pub unlock_time: u64,
    pub timestamp: u64,
    pub version: u8,
    pub payment_id: CryptoHash,
    pub has_payment_id: bool,
    pub in_blockchain: bool,
    pub block_hash: CryptoHash,
    pub block_height: u32,
    pub extra: TransactionExtraDetails2,
    pub signatures: Vec<Vec<CryptoSignature>>,
    pub inputs: Vec<TransactionInputDetails2>,
    pub outputs: Vec<TransactionOutputDetails2>,
    // non-privacy fields:
    pub from_address: String,
    pub to_address: Vec<String>,
    pub amount: Vec<String>,
    pub tx_key: CryptoSecretKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockDetails {
    pub major_version: u8,
    pub minor_version: u8,
    pub timestamp: u64,
    pub prev_block_hash: CryptoHash,
    pub proof_of_work: CryptoHash,
    pub nonce: u32,
    pub is_orphaned: bool,
    pub height: u32,
    pub depth: u32,
    pub hash: CryptoHash,
    pub difficulty: u64,
    pub cumulative_difficulty: u64,
    pub reward: u64,
    pub base_reward: u64,
    pub block_size: u64,
    pub transactions_cumulative_size: u64,
    pub already_generated_coins: u64,
    pub already_generated_transactions: u64,
    pub size_median: u64,
    pub effective_size_median: u64,
    pub penalty: f64,
    pub total_fee_amount: u64,
    pub transactions: Vec<TransactionDetails>,
}

// Placeholder types for external dependencies
pub type CryptoHash = Vec<u8>;
pub type CryptoPublicKey = Vec<u8>;
pub type CryptoSecretKey = Vec<u8>;
pub type CryptoSignature = Vec<u8>;
pub type CryptoKeyImage = Vec<u8>;
pub type BinaryArray = Vec<u8>;

pub struct TransactionOutput;
pub struct BaseInput;
pub struct KeyInput;
pub struct MultisignatureInput;



