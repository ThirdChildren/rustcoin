use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid private key")]
    InvalidPrivateKey,
    #[error("Invalid block")]
    InvalidBlock,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid transaction")]
    InvalidTransaction,
    #[error("Invalid block header")]
    InvalidBlockHeader,
    #[error("Invalid transaction input")]
    InvalidTransactionInput,
    #[error("Invalid transaction output")]
    InvalidTransactionOutput,
    #[error("Invalid Merkle root")]
    InvalidMerkleRoot,
}

pub type Result<T> = std::result::Result<T, BtcError>;
