use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    tx: TransactionDetails,
    stx_sent: String,
    stx_received: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionDetails {
    tx_id: String,
    nonce: u64,
    fee_rate: String,
    sender_address: String,
    sponsored: bool,
    block_hash: String,
    block_height: u64,
    block_time: u64,
    block_time_iso: String,
    burn_block_time: u64,
    burn_block_height: u64,
    burn_block_time_iso: String,
    parent_burn_block_time: u64,
    parent_burn_block_time_iso: String,
    canonical: bool,
    tx_index: u64,
    tx_status: String,
    tx_result: TxResult,
    event_count: u64,
    parent_block_hash: String,
    is_unanchored: bool,
    microblock_hash: String,
    microblock_sequence: u64,
    microblock_canonical: bool,
    tx_type: String,
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
struct ApiResponse {
    results: Vec<Transaction>,
    total: u64,
    limit: u64,
    offset: u64,
}

const BASE_URL: &str = "https://api.hiro.so/extended/v2/addresses";

/// Fetches all transactions for a given Stacks address
pub async fn fetch_all_transactions(address: &str) -> Result<Vec<Transaction>, Error> {
    let mut all_transactions: Vec<Transaction> = Vec::new();
    let mut offset: u64 = 0;
    let limit = 20;

    loop {
        // Construct the URL with the current offset and limit
        let url = format!("{}/{}/transactions?limit={}&offset={}", BASE_URL, address, limit, offset);

        // Send the request
        // let response: ApiResponse = reqwest::get(&url).await?.json().await?;
        let response = match reqwest::get(&url).await?.json::<ApiResponse>().await {
            Ok(api_response) => api_response,
            Err(e) => {
                eprintln!("Error fetching or parsing response from {}: {:?}", url, e);
                break; // Stop the loop if there's an error
            }
        };

        // Add the fetched transactions to the total list

        if response.results.is_empty() {
            break;
        }

        all_transactions.extend(response.results);

        // Update offset for the next page
        offset += limit;

    }

    Ok(all_transactions)
}
