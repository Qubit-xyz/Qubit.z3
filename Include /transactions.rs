use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

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
    pub struct TransactionPrefix;

    #[derive(Debug, Clone)]
    pub struct AccountPublicAddress;

    #[derive(Debug, Clone)]
    pub struct KeyInput;

    #[derive(Debug, Clone)]
    pub struct MultisignatureInput;

    #[derive(Debug, Clone)]
    pub struct KeyOutput;

    #[derive(Debug, Clone)]
    pub struct MultisignatureOutput;

    #[derive(Debug, Clone)]
    pub struct BinaryArray(pub Vec<u8>);

    #[derive(Debug, Clone)]
    pub struct TransactionDetails;

    
    pub mod TransactionTypes {
        #[derive(Debug, Clone, Copy)]
        pub enum InputType {
            Invalid = 0,
            Key = 1,
            Multisignature = 2,
            Generating = 3,
        }

        #[derive(Debug, Clone, Copy)]
        pub enum OutputType {
            Invalid = 0,
            Key = 1,
            Multisignature = 2,
        }

        #[derive(Debug, Clone)]
        pub struct GlobalOutput {
            pub target_key: super::PublicKey,
            pub output_index: u32,
        }

        pub type GlobalOutputsContainer = Vec<GlobalOutput>;

        #[derive(Debug, Clone)]
        pub struct OutputKeyInfo {
            pub transaction_public_key: super::PublicKey,
            pub transaction_index: usize,
            pub output_in_transaction: usize,
        }

        #[derive(Debug, Clone)]
        pub struct InputKeyInfo {
            pub amount: u64,
            pub outputs: GlobalOutputsContainer,
            pub real_output: OutputKeyInfo,
        }
    }

    
    pub trait ITransactionReader {
        fn get_transaction_hash(&self) -> CryptoHash;
        fn get_transaction_prefix_hash(&self) -> CryptoHash;
        fn get_transaction_public_key(&self) -> PublicKey;
        fn get_transaction_secret_key(&self) -> Option<SecretKey>;
        fn get_unlock_time(&self) -> u64;

        
        fn get_payment_id(&self) -> Option<CryptoHash>;
        fn get_extra_nonce(&self) -> Option<BinaryArray>;
        fn get_extra(&self) -> BinaryArray;

        
        fn get_input_count(&self) -> usize;
        fn get_input_total_amount(&self) -> u64;
        fn get_input_type(&self, index: usize) -> TransactionTypes::InputType;
        fn get_input(&self, index: usize) -> Option<KeyInput>;
        fn get_input_multisignature(&self, index: usize) -> Option<MultisignatureInput>;

        
        fn get_output_count(&self) -> usize;
        fn get_output_total_amount(&self) -> u64;
        fn get_output_type(&self, index: usize) -> TransactionTypes::OutputType;
        fn get_output(&self, index: usize) -> Option<(KeyOutput, u64)>;
        fn get_output_multisignature(&self, index: usize) -> Option<(MultisignatureOutput, u64)>;

        
        fn get_required_signatures_count(&self, input_index: usize) -> usize;
        fn find_outputs_to_account(
            &self,
            addr: &AccountPublicAddress,
            view_secret_key: &SecretKey,
        ) -> Option<(Vec<u32>, u64)>;

        
        fn validate_inputs(&self) -> bool;
        fn validate_outputs(&self) -> bool;
        fn validate_signatures(&self) -> bool;

        
        fn get_transaction_data(&self) -> BinaryArray;
    }

    
    pub trait ITransactionWriter {
        fn set_unlock_time(&mut self, unlock_time: u64);

        
        fn set_payment_id(&mut self, payment_id: CryptoHash);
        fn set_extra_nonce(&mut self, nonce: BinaryArray);
        fn append_extra(&mut self, extra_data: BinaryArray);

        
        fn add_input(&mut self, input: KeyInput) -> usize;
        fn add_input_multisignature(&mut self, input: MultisignatureInput) -> usize;
        fn add_input_with_info(
            &mut self,
            sender_keys: &AccountKeys,
            info: &TransactionTypes::InputKeyInfo,
            eph_keys: &KeyPair,
        ) -> usize;

        fn add_output(&mut self, amount: u64, to: &AccountPublicAddress) -> usize;
        fn add_output_with_signatures(
            &mut self,
            amount: u64,
            to: &Vec<AccountPublicAddress>,
            required_signatures: u32,
        ) -> usize;
        fn add_output_key(&mut self, amount: u64, output: KeyOutput) -> usize;
        fn add_output_multisignature(&mut self, amount: u64, output: MultisignatureOutput) -> usize;

        
        fn set_transaction_secret_key(&mut self, key: SecretKey);

        
        fn sign_input_key(
            &mut self,
            input: usize,
            info: &TransactionTypes::InputKeyInfo,
            eph_keys: &KeyPair,
        );

        fn sign_input_multisignature(
            &mut self,
            input: usize,
            source_transaction_key: &PublicKey,
            output_index: usize,
            account_keys: &AccountKeys,
        );

        fn sign_input_multisignature_ephemeral_keys(
            &mut self,
            input: usize,
            ephemeral_keys: &KeyPair,
        );
    }

    
    pub trait ITransaction: ITransactionReader + ITransactionWriter {
        fn new() -> Self;
    }

    
    #[derive(Debug, Clone)]
    pub struct AccountKeys;
}

