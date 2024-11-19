use crate::stacks::utils::{Transaction, TransactionDetails};

pub fn pad_to_power_of_two(transactions: Vec<Transaction>) -> Vec<Transaction> {
    let current_length = transactions.len();
    let next_power_of_two = current_length.next_power_of_two();

    let dummy_transaction = Transaction {
        stx_received: "0".to_string(),
        stx_sent: "0".to_string(),
        tx: TransactionDetails {
            tx_id: "0".to_string(),
            burn_block_height: 0,
            nonce: 0,
            block_height: 0,
            tx_index: 0,
            tx_status: "".to_string(),
            parent_block_hash: "".to_string(),
            tx_type: "".to_string(),
        },
    };

    // Create a new vector with the original transactions
    let mut padded_transactions = transactions;

    // Add dummy transactions to match the length to the next power of two
    padded_transactions.extend((0..(next_power_of_two - current_length)).map(|_| dummy_transaction.clone()));

    padded_transactions
}
