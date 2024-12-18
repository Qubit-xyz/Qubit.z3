use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;
use std::time::SystemTime;
use boost::optional::Optional; 

pub mod DynexCN {
    
    #[derive(Debug, Clone)]
    pub struct CryptoHash(pub Vec<u8>);

    #[derive(Debug, Clone)]
    pub struct PublicKey(pub Vec<u8>);

    #[derive(Debug, Clone)]
    pub struct SecretKey(pub Vec<u8>);

    #[derive(Debug, Clone)]
    pub struct KeyPair {
        pub public: PublicKey,
        pub secret: SecretKey,
    }

    #[derive(Debug, Clone)]
    pub struct AccountPublicAddress;

    
    pub const WALLET_INVALID_TRANSACTION_ID: usize = std::usize::MAX;
    pub const WALLET_INVALID_TRANSFER_ID: usize = std::usize::MAX;
    pub const WALLET_UNCONFIRMED_TRANSACTION_HEIGHT: u32 = std::u32::MAX;

    
    #[derive(Debug, Clone, Copy)]
    pub enum WalletTransactionState {
        SUCCEEDED = 0,
        FAILED,
        CANCELLED,
        CREATED,
        DELETED,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum WalletEventType {
        TRANSACTION_CREATED,
        TRANSACTION_UPDATED,
        BALANCE_UNLOCKED,
        SYNC_PROGRESS_UPDATED,
        SYNC_COMPLETED,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum WalletSaveLevel {
        SAVE_KEYS_ONLY,
        SAVE_KEYS_AND_TRANSACTIONS,
        SAVE_ALL,
    }

    #[derive(Debug, Clone)]
    pub struct WalletTransactionCreatedData {
        pub transaction_index: usize,
    }

    #[derive(Debug, Clone)]
    pub struct WalletTransactionUpdatedData {
        pub transaction_index: usize,
    }

    #[derive(Debug, Clone)]
    pub struct WalletSynchronizationProgressUpdated {
        pub processed_block_count: u32,
        pub total_block_count: u32,
    }

    #[derive(Debug, Clone)]
    pub struct WalletEvent {
        pub event_type: WalletEventType,
        pub data: WalletEventData,
    }

    #[derive(Debug, Clone)]
    pub enum WalletEventData {
        TransactionCreated(WalletTransactionCreatedData),
        TransactionUpdated(WalletTransactionUpdatedData),
        SynchronizationProgressUpdated(WalletSynchronizationProgressUpdated),
    }

    #[derive(Debug, Clone)]
    pub struct WalletTransaction {
        pub state: WalletTransactionState,
        pub timestamp: u64,
        pub block_height: u32,
        pub hash: CryptoHash,
        pub secret_key: Option<SecretKey>,
        pub total_amount: i64,
        pub fee: u64,
        pub creation_time: u64,
        pub unlock_time: u64,
        pub extra: String,
        pub is_base: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum WalletTransferType {
        USUAL = 0,
        DONATION,
        CHANGE,
    }

    #[derive(Debug, Clone)]
    pub struct WalletOrder {
        pub address: String,
        pub amount: u64,
    }

    #[derive(Debug, Clone)]
    pub struct WalletTransfer {
        pub transfer_type: WalletTransferType,
        pub address: String,
        pub amount: i64,
    }

    #[derive(Debug, Clone)]
    pub struct DonationSettings {
        pub address: String,
        pub threshold: u64,
    }

    #[derive(Debug, Clone)]
    pub struct TransactionParameters {
        pub source_addresses: Vec<String>,
        pub destinations: Vec<WalletOrder>,
        pub fee: u64,
        pub mix_in: u64,
        pub extra: String,
        pub unlock_timestamp: u64,
        pub donation: DonationSettings,
        pub change_destination: String,
    }

    #[derive(Debug, Clone)]
    pub struct WalletTransactionWithTransfers {
        pub transaction: WalletTransaction,
        pub transfers: Vec<WalletTransfer>,
    }

    #[derive(Debug, Clone)]
    pub struct TransactionsInBlockInfo {
        pub block_hash: CryptoHash,
        pub transactions: Vec<WalletTransactionWithTransfers>,
    }

    
    pub trait IWallet {
        fn initialize(&mut self, path: &str, password: &str);
        fn initialize_with_view_key(&mut self, path: &str, password: &str, view_secret_key: &SecretKey);
        fn load(&mut self, path: &str, password: &str, extra: &mut String);
        fn load_simple(&mut self, path: &str, password: &str);
        fn shutdown(&mut self);
        fn change_password(&mut self, old_password: &str, new_password: &str);
        fn save(&mut self, save_level: WalletSaveLevel, extra: &str);
        fn reset(&mut self, scan_height: u64);
        fn export_wallet(&mut self, path: &str, encrypt: bool, save_level: WalletSaveLevel, extra: &str);

        fn get_address_count(&self) -> usize;
        fn get_address(&self, index: usize) -> String;
        fn get_account_public_address(&self, index: usize) -> AccountPublicAddress;
        fn get_address_spend_key(&self, index: usize) -> KeyPair;
        fn get_address_spend_key_by_address(&self, address: &str) -> KeyPair;
        fn get_view_key(&self) -> KeyPair;
        fn create_address(&mut self) -> String;
        fn create_address_with_spend_key(&mut self, spend_secret_key: &SecretKey, reset: bool) -> String;
        fn create_address_with_public_key(&mut self, spend_public_key: &PublicKey, reset: bool) -> String;
        fn get_actual_balance(&self) -> u64;
        fn get_pending_balance(&self) -> u64;
        fn get_transaction_count(&self) -> usize;
        fn get_transaction(&self, transaction_index: usize) -> WalletTransaction;
        fn get_transaction_secret_key(&self, transaction_index: usize) -> SecretKey;
        fn get_transaction_by_hash(&self, transaction_hash: &CryptoHash) -> WalletTransactionWithTransfers;
        fn get_transactions_in_block(&self, block_hash: &CryptoHash, count: usize) -> Vec<TransactionsInBlockInfo>;
        fn get_block_hashes(&self, block_index: u32, count: usize) -> Vec<CryptoHash>;
        fn get_block_count(&self) -> u32;
        fn get_unconfirmed_transactions(&self) -> Vec<WalletTransactionWithTransfers>;
        fn get_delayed_transaction_ids(&self) -> Vec<usize>;
        fn get_transfers(&self, index: usize, flags: u32) -> Vec<TransactionOutputInformation>;
        fn get_reserve_proof(&self, reserve: u64, address: &str, message: &str) -> String;
        fn get_spendable_outputs(&self, address: &str) -> String;

        fn transfer(&mut self, sending_transaction: &TransactionParameters, tx_secret_key: &mut SecretKey) -> usize;
        fn make_transaction(&mut self, sending_transaction: &TransactionParameters) -> usize;
        fn commit_transaction(&mut self, transaction_id: usize);
        fn rollback_uncommitted_transaction(&mut self, transaction_id: usize);

        fn start(&mut self);
        fn stop(&mut self);

        fn get_event(&mut self) -> WalletEvent;
    }

    
    #[derive(Debug, Clone)]
    pub struct TransactionOutputInformation;
}
