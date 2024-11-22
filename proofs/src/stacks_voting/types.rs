use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Domain {
    name: String,
    version: String,
    chain_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct MessageInputs {
    pub message: String,
    pub vote: String,
    pub proposal: String,
    pub balance_at_height: u64,
    pub block_proof_height: u64,
    pub voting_end_height: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignatureData {
    pub  message_inputs: MessageInputs,
    pub public_key: String,
    pub hash: String,
    pub signature: String,
    pub message: String,
    pub domain: Option<Domain>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub tx: TransactionDetails,
    pub stx_sent: String,
    pub stx_received: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionDetails {
    pub tx_id: String,
    pub nonce: u64,
    // fee_rate: String,
    // sender_address: String,
    // sponsored: bool,
    // block_hash: String,
    pub block_height: u64,
    // block_time: u64,
    // block_time_iso: String,
    // burn_block_time: u64,
    pub burn_block_height: u64,
    // burn_block_time_iso: String,
    // parent_burn_block_time: u64,
    // parent_burn_block_time_iso: String,
    // canonical: bool,
    pub tx_index: u64,
    pub tx_status: String,
    // tx_result: TxResult,
    // event_count: u64,
    pub parent_block_hash: String,
    // is_unanchored: bool,
    // microblock_hash: String,
    // microblock_sequence: u64,
    // microblock_canonical: bool,
    pub tx_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxResult {
    hex: String,
    repr: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    stx: StxEvent,
    ft: FtEvent,
    nft: FtEvent
}
#[derive(Deserialize, Serialize, Debug)]
pub struct StxEvent {
    transfer: u16,
    mint: u16,
    burn: u16
}
#[derive(Deserialize, Serialize, Debug)]
pub struct FtEvent {
    transfer: u16,
    mint: u16,
    burn: u16
}
#[derive(Deserialize, Serialize, Debug)]
pub struct NftEvent {
    transfer: u16,
    mint: u16,
    burn: u16
}


#[derive(Deserialize, Serialize, Debug)]
pub struct TokenTransfer {
    recipient_address: String,
    amount: String,
    memo: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub results: Vec<Transaction>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}

