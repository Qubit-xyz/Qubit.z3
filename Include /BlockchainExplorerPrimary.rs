use std::vec::Vec;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BlockDetails;  
#[derive(Debug, Clone)]
pub struct TransactionDetails;  
#[derive(Debug, Clone)]
pub struct CryptoHash(pub Vec<u8>); 
#[derive(Debug, Clone)]
pub enum TransactionRemoveReason {
    INCLUDED_IN_BLOCK,
    TIMEOUT,
}

pub trait IBlockchainObserver {
    fn blockchain_updated(&self, new_blocks: Vec<BlockDetails>, orphaned_blocks: Vec<BlockDetails>);
    fn pool_updated(&self, new_transactions: Vec<TransactionDetails>, removed_transactions: Vec<(CryptoHash, TransactionRemoveReason)>);
    fn blockchain_synchronized(&self, top_block: BlockDetails);
}

pub trait IBlockchainExplorer {
    fn add_observer(&mut self, observer: Box<dyn IBlockchainObserver>) -> bool;
    fn remove_observer(&mut self, observer: &dyn IBlockchainObserver) -> bool;

    fn init(&mut self);
    fn shutdown(&mut self);

    fn get_blocks_by_heights(&self, block_heights: Vec<u32>) -> Vec<BlockDetails>;
    fn get_blocks_by_hashes(&self, block_hashes: Vec<CryptoHash>) -> Vec<BlockDetails>;
    fn get_blocks_by_time_range(
        &self, 
        timestamp_begin: u64, 
        timestamp_end: u64, 
        blocks_number_limit: u32
    ) -> (Vec<BlockDetails>, u32);  
    
    fn get_blockchain_top(&self) -> Option<BlockDetails>;

    fn get_transactions_by_hashes(&self, transaction_hashes: Vec<CryptoHash>) -> Vec<TransactionDetails>;
    fn get_transactions_by_payment_id(&self, payment_id: CryptoHash) -> Vec<TransactionDetails>;
    fn get_pool_transactions(
        &self, 
        timestamp_begin: u64, 
        timestamp_end: u64, 
        transactions_number_limit: u32
    ) -> (Vec<TransactionDetails>, u64);  

    fn get_pool_state(
        &self, 
        known_pool_transaction_hashes: Vec<CryptoHash>, 
        known_blockchain_top: CryptoHash
    ) -> (bool, Vec<TransactionDetails>, Vec<CryptoHash>);  

    fn get_reward_blocks_window(&self) -> u64;
    fn get_full_reward_max_block_size(&self, major_version: u8) -> u64;

    fn is_synchronized(&self) -> bool;
}

