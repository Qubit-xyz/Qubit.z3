use std::error::Error;
use std::vec::Vec;
use std::string::String;
use std::sync::Arc;
use std::sync::Mutex;

pub mod DynexCN {
    use super::*;
    
    
    #[derive(Debug, Clone)]
    pub struct CryptoHash(pub Vec<u8>);
    
    #[derive(Debug, Clone)]
    pub struct TransactionPrefix;
    
    #[derive(Debug, Clone)]
    pub struct Block;
    
    #[derive(Debug, Clone)]
    pub struct Transaction;
    
    #[derive(Debug, Clone)]
    pub struct BlockDetails;
    
    #[derive(Debug, Clone)]
    pub struct TransactionDetails;
    
    pub type difficulty_type = u64;

    #[derive(Debug, Clone)]
    pub struct MultisignatureOutput;
    
    #[derive(Debug, Clone)]
    pub struct OutEntry {
        pub out_global_index: u32,
        pub out_key: Crypto::PublicKey,
    }
    
    #[derive(Debug, Clone)]
    pub struct OutsForAmount {
        pub amount: u64,
        pub outs: Vec<OutEntry>,
    }
    
    #[derive(Debug, Clone)]
    pub struct TransactionShortInfo {
        pub tx_id: Crypto::Hash,
        pub tx_prefix: TransactionPrefix,
    }
    
    #[derive(Debug, Clone)]
    pub struct BlockShortEntry {
        pub block_hash: Crypto::Hash,
        pub has_block: bool,
        pub block: Block,
        pub txs_short_info: Vec<TransactionShortInfo>,
    }
    
    #[derive(Debug, Clone)]
    pub struct BlockHeaderInfo {
        pub index: u32,
        pub major_version: u8,
        pub minor_version: u8,
        pub timestamp: u64,
        pub hash: Crypto::Hash,
        pub prev_hash: Crypto::Hash,
        pub nonce: u32,
        pub is_alternative: bool,
        pub depth: u32,
        pub difficulty: difficulty_type,
        pub reward: u64,
    }
    
    
    pub type Callback = Box<dyn FnOnce(Result<(), Box<dyn Error>>) + Send>;
    
    
    pub trait INodeObserver {
        fn peer_count_updated(&self, count: usize);
        fn local_blockchain_updated(&self, height: u32);
        fn last_known_block_height_updated(&self, height: u32);
        fn pool_changed(&self);
        fn blockchain_synchronized(&self, top_height: u32);
    }
    
    
    pub trait INode {
        fn add_observer(&mut self, observer: Arc<dyn INodeObserver>) -> bool;
        fn remove_observer(&mut self, observer: Arc<dyn INodeObserver>) -> bool;
        fn init(&self, callback: Callback);
        fn shutdown(&self) -> bool;

        fn get_peer_count(&self) -> usize;
        fn get_last_local_block_height(&self) -> u32;
        fn get_last_known_block_height(&self) -> u32;
        fn get_local_block_count(&self) -> u32;
        fn get_known_block_count(&self) -> u32;
        fn get_minimal_fee(&self) -> u64;
        fn get_last_local_block_timestamp(&self) -> u64;
        fn get_node_height(&self) -> u32;
        fn get_last_local_block_header_info(&self) -> BlockHeaderInfo;

        fn get_fee_address(&self) -> String;

        fn relay_transaction(&self, transaction: &Transaction, callback: Callback);
        fn get_random_outs_by_amounts(
            &self, 
            amounts: Vec<u64>, 
            outs_count: u64, 
            result: &mut Vec<OutsForAmount>, 
            callback: Callback
        );
        fn get_new_blocks(
            &self, 
            known_block_ids: Vec<Crypto::Hash>, 
            new_blocks: &mut Vec<BlockShortEntry>, 
            start_height: &mut u32, 
            callback: Callback
        );
        fn get_transaction_outs_global_indices(
            &self, 
            transaction_hash: Crypto::Hash, 
            outs_global_indices: &mut Vec<u32>, 
            callback: Callback
        );
        fn query_blocks(
            &self, 
            known_block_ids: Vec<Crypto::Hash>, 
            timestamp: u64, 
            new_blocks: &mut Vec<BlockShortEntry>, 
            start_height: &mut u32, 
            callback: Callback
        );
        fn get_pool_symmetric_difference(
            &self, 
            known_pool_tx_ids: Vec<Crypto::Hash>, 
            known_block_id: Crypto::Hash, 
            is_bc_actual: &mut bool, 
            new_txs: &mut Vec<Arc<dyn ITransactionReader>>, 
            deleted_tx_ids: &mut Vec<Crypto::Hash>, 
            callback: Callback
        );
        fn get_multisignature_output_by_global_index(
            &self, 
            amount: u64, 
            global_index: u32, 
            out: &mut MultisignatureOutput, 
            callback: Callback
        );
        
        fn get_blocks(
            &self, 
            block_heights: Vec<u32>, 
            blocks: &mut Vec<Vec<BlockDetails>>, 
            callback: Callback
        );
        fn get_blocks_by_hashes(
            &self, 
            block_hashes: Vec<Crypto::Hash>, 
            blocks: &mut Vec<BlockDetails>, 
            callback: Callback
        );
        fn get_blocks_by_time_range(
            &self, 
            timestamp_begin: u64, 
            timestamp_end: u64, 
            blocks_number_limit: u32, 
            blocks: &mut Vec<BlockDetails>, 
            blocks_number_within_timestamps: &mut u32, 
            callback: Callback
        );
        fn get_block(
            &self, 
            block_height: u32, 
            block: &mut BlockDetails, 
            callback: Callback
        );
        fn get_transactions(
            &self, 
            transaction_hashes: Vec<Crypto::Hash>, 
            transactions: &mut Vec<TransactionDetails>, 
            callback: Callback
        );
        fn get_transactions_by_payment_id(
            &self, 
            payment_id: Crypto::Hash, 
            transactions: &mut Vec<TransactionDetails>, 
            callback: Callback
        );
        fn get_pool_transactions(
            &self, 
            timestamp_begin: u64, 
            timestamp_end: u64, 
            transactions_number_limit: u32, 
            transactions: &mut Vec<TransactionDetails>, 
            transactions_number_within_timestamps: &mut u64, 
            callback: Callback
        );
        fn is_synchronized(
            &self, 
            sync_status: &mut bool, 
            callback: Callback
        );
        fn fee_address(&self) -> String;
    }

 
    pub trait ITransactionReader {
        fn read_transaction(&self);
    }
}
