use std::error::Error;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::time::SystemTime;

mod DynexCN {
    use super::Crypto;
    
    
    pub type TransactionId = usize;
    pub type TransferId = usize;

    pub const WALLET_LEGACY_INVALID_TRANSACTION_ID: TransactionId = std::usize::MAX;
    pub const WALLET_LEGACY_INVALID_TRANSFER_ID: TransferId = std::usize::MAX;
    pub const WALLET_LEGACY_UNCONFIRMED_TRANSACTION_HEIGHT: u32 = std::u32::MAX;

    #[derive(Debug, Clone, Copy)]
    pub enum WalletLegacyTransactionState {
        Active,
        Deleted,
        Sending,
        Cancelled,
        Failed,
    }

    #[derive(Debug, Clone)]
    pub struct WalletLegacyTransaction {
        pub first_transfer_id: TransferId,
        pub transfer_count: usize,
        pub total_amount: i64,
        pub fee: u64,
        pub sent_time: u64,
        pub unlock_time: u64,
        pub hash: Crypto::Hash,
        pub secret_key: Option<Crypto::SecretKey>,
        pub is_coinbase: bool,
        pub block_height: u32,
        pub timestamp: u64,
        pub extra: String,
        pub state: WalletLegacyTransactionState,
    }

    #[derive(Debug, Clone)]
    pub struct WalletLegacyTransfer {
        pub address: String,
        pub amount: i64,
    }

    pub type PaymentId = Crypto::Hash;
    
    #[derive(Debug, Clone)]
    pub struct Payments {
        pub payment_id: PaymentId,
        pub transactions: Vec<WalletLegacyTransaction>,
    }

    
    pub const _: () = assert!(std::mem::size_of::<Payments>() > 0, "Payments is not move constructible");

    
    pub trait IWalletLegacyObserver {
        fn init_completed(&self, result: io::Result<()>);
        fn save_completed(&self, result: io::Result<()>);
        fn synchronization_progress_updated(&self, current: u32, total: u32);
        fn synchronization_completed(&self, result: io::Result<()>);
        fn actual_balance_updated(&self, actual_balance: u64);
        fn pending_balance_updated(&self, pending_balance: u64);
        fn unmixable_balance_updated(&self, dust_balance: u64);
        fn external_transaction_created(&self, transaction_id: TransactionId);
        fn send_transaction_completed(&self, transaction_id: TransactionId, result: io::Result<()>);
        fn transaction_updated(&self, transaction_id: TransactionId);
    }

    
    pub trait IWalletLegacy {
        fn add_observer(&mut self, observer: Box<dyn IWalletLegacyObserver>);
        fn remove_observer(&mut self, observer: Box<dyn IWalletLegacyObserver>);

        fn init_and_generate(&mut self, password: &str);
        fn init_and_generate_deterministic(&mut self, password: &str);
        fn generate_key(&mut self, password: &str, recovery_param: Option<Crypto::SecretKey>, recover: bool, two_random: bool) -> Crypto::SecretKey;
        fn init_and_load(&mut self, source: &mut dyn Read, password: &str);
        fn init_with_keys(&mut self, account_keys: &AccountKeys, password: &str);
        fn init_with_keys_and_scan_height(&mut self, account_keys: &AccountKeys, password: &str, scan_height: u32);
        fn shutdown(&mut self);
        fn reset(&mut self);
        
        fn save(&self, destination: &mut dyn Write, save_detailed: bool, save_cache: bool);
        
        fn change_password(&mut self, old_password: &str, new_password: &str) -> io::Result<()>;

        fn get_address(&self) -> String;
        
        fn actual_balance(&self) -> u64;
        fn pending_balance(&self) -> u64;
        fn dust_balance(&self) -> u64;

        fn get_transaction_count(&self) -> usize;
        fn get_transfer_count(&self) -> usize;
        fn get_unlocked_outputs_count(&self) -> usize;
        
        fn get_unlocked_outputs(&self) -> Vec<TransactionOutputInformation>;
        
        fn sign_transaction(&mut self, tx: &mut Transaction, tx_key: Crypto::SecretKey, amount: u64, fee: u64) -> TransactionId;
        
        fn find_transaction_by_transfer_id(&self, transfer_id: TransferId) -> Option<TransactionId>;
        
        fn get_transaction(&self, transaction_id: TransactionId) -> Option<WalletLegacyTransaction>;
        fn get_transfer(&self, transfer_id: TransferId) -> Option<WalletLegacyTransfer>;
        
        fn get_transactions_by_payment_ids(&self, payment_ids: Vec<PaymentId>) -> Vec<Payments>;
        
        fn get_tx_proof(&self, tx_id: &Crypto::Hash, address: &AccountPublicAddress, tx_key: &Crypto::SecretKey) -> io::Result<String>;
        
        fn get_reserve_proof(&self, reserve: u64, message: &str) -> String;
        fn get_tx_key(&self, tx_id: &Crypto::Hash) -> Crypto::SecretKey;
        
        fn get_account_keys(&self) -> AccountKeys;
        fn get_seed(&self) -> Option<String>;

        fn send_transaction(&mut self, transfer: &WalletLegacyTransfer, fee: u64, extra: Option<String>, mix_in: u64, unlock_timestamp: u64) -> TransactionId;
        fn send_transactions(&mut self, transfers: Vec<WalletLegacyTransfer>, fee: u64, extra: Option<String>, mix_in: u64, unlock_timestamp: u64) -> TransactionId;
        fn send_dust_transaction(&mut self, transfers: Vec<WalletLegacyTransfer>, fee: u64, extra: Option<String>, mix_in: u64, unlock_timestamp: u64) -> TransactionId;
        fn send_fusion_transaction(&mut self, fusion_inputs: Vec<TransactionOutputInformation>, fee: u64, extra: Option<String>, mix_in: u64, unlock_timestamp: u64) -> TransactionId;
        
        fn cancel_transaction(&mut self, transfer_id: TransferId) -> io::Result<()>;

        fn estimate_fusion(&self, threshold: u64) -> usize;
        fn select_fusion_transfers_to_send(&self, threshold: u64, min_input_count: usize, max_input_count: usize) -> Vec<TransactionOutputInformation>;

        fn get_transaction_information(&self, transaction_hash: &Crypto::Hash) -> Option<TransactionInformation>;
        fn get_transaction_outputs(&self, transaction_hash: &Crypto::Hash) -> Vec<TransactionOutputInformation>;
        fn get_transaction_inputs(&self, transaction_hash: &Crypto::Hash) -> Vec<TransactionInputInformation>;

        fn is_fusion_transaction(&self, wallet_tx: &WalletLegacyTransaction) -> bool;

        fn sign_message(&self, data: &str) -> String;
        fn verify_message(&self, data: &str, address: &AccountPublicAddress, signature: &str) -> bool;

        fn is_tracking_wallet(&self) -> bool;
    }

    
    pub struct AccountKeys;
    pub struct TransactionOutputInformation;
    pub struct TransactionInputInformation;
    pub struct TransactionInformation;
    pub struct Transaction;
    pub struct Crypto {
        pub struct SecretKey;
        pub struct Hash;
    }
    
    pub struct AccountPublicAddress;
}
