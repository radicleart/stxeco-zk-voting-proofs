use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use stacks_rs::crypto::c32_address;

use crate::ProofsError;

use super::{types::Transaction, utils::fetch_and_parse};

const BASE_URL: &str = "https://api.hiro.so/extended/v2/addresses";

/// Fetches all transactions for a given Stacks address
pub async fn fetch_all_transactions(address: &str) -> Result<Vec<Transaction>, ProofsError> {
    let mut all_transactions: Vec<Transaction> = Vec::new();
    let mut offset: u64 = 0;
    let limit = 20;

    loop {
        let url = format!("{}/{}/transactions?limit={}&offset={}", BASE_URL, address, limit, offset);
        match fetch_and_parse(&url).await {
            Ok(response) => {
                println!("Received response: {:?}", response);        
                if response.results.is_empty() {
                    break;
                }
                all_transactions.extend(response.results);
                offset += limit;
            },
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    Ok(all_transactions)
}


pub fn public_key_to_stacks_address(public_key_hex: String) -> Result<String, ProofsError> {
    // Step 1: Decode the hex string of the public key
    let public_key_bytes = hex::decode(&public_key_hex).map_err(|e| ProofsError::new(&format!("Error parsing public_key_bytes {}: {}", &public_key_hex, e)))?;
    // Step 2: Hash the public key with SHA-256
    let sha256_hash = Sha256::digest(&public_key_bytes);
    // Step 3: Hash the result with RIPEMD-160
    let mut hasher = Ripemd160::new();
    hasher.update(sha256_hash);
    let ripemd160_hash = hasher.finalize();
    // Step 4: Convert the hash to a C32Check Stacks address (version 22 for mainnet)
    let version_byte = 22; // 22 is the mainnet version byte for Stacks addresses
    let ripemd160_hash_array: [u8; 20] = ripemd160_hash.into();

    let stacks_address = c32_address(ripemd160_hash_array, version_byte).map_err(|e| ProofsError::new(&format!("Error parsing public_key_bytes {}: {}", public_key_hex, e)))?;
    println!("Stacks address: {}", stacks_address);
    Ok(stacks_address)
}
