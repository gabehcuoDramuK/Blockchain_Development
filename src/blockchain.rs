use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::block::Block;

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub balances: HashMap<String, i32>, // Stores balances for each account
    pub difficulty: usize,
}

impl Blockchain {
    /**
     * Create a new blockchain with a genesis block of type Blockchain.
     * difficulty is the number of leading zeros required in the hash of a block to be considered valid and it should be set to 4 by default.
     */
    pub fn new(difficulty: usize) -> Self {
        // TODO: Create a new blockchain with a genesis block
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            balances: HashMap::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let mut genesis_block = Block::new(
            0,
            "0".to_string(),
            "Genesis Block".to_string(),
            self.difficulty,
            current_timestamp(),
        );

        genesis_block.mine(self.difficulty); //Ensures mining is done properly!

        self.chain.push(genesis_block);
    }

    pub fn last_block(&self) -> &Block {
        // TODO: Return the last block in the chain
        self.chain.last().expect("Blockchain Must have atleast one block")
    }

    pub fn add_block(&mut self, data: String) {

        // Creates a new block with the provided data and add it to the chain
        let last_block = self.last_block();

        let data_clone = data.clone(); // Clone data before moving it!

        let mut new_block = Block::new(
            last_block.index + 1,
            last_block.hash.clone(),
            data_clone,
            self.difficulty,
            current_timestamp()
            );

        new_block.hash = new_block.mine(self.difficulty); // Mine the block before adding!

        /* To display the entire Block Info
           println!("\n[New Block Added] Index: {} | Nonce: {}", new_block.index, new_block.nonce);
           println!("Previous Hash: {}", last_block.hash);
           println!("Current Hash: {}", new_block.hash);
           println!("Block Data: {}\n", data);
        */

        self.chain.push(new_block);
    }


    pub fn is_valid(&self) -> bool {
        // Verifying the integrity of the whole blockchain by checking
        // each block’s hash and its connection to the previous block.
        // Return true if the blockchain is valid, false otherwise.

        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i -1];

            if current_block.hash != current_block.calculate_hash() {

                println!("Blockchain is Invalid at Block {} | Expected hash: {} | Found: {}", current_block.index, current_block.calculate_hash(), current_block.hash);

                return false;
            }

            if current_block.previous_hash != previous_block.hash {

                println!("Blockchain is Invalid at Block {} | Previous Block Mismatch!", current_block.index);
                return false;
            }

        }
        println!("\nBlockchain is Valid.\n");
        true
    
    }

    pub fn transfer_balance(&mut self, sender: String, receiver: String, amount: i32) -> bool {

        /* Below code snippet implements the transfer_balance method by transferring
           the amount from the sender to the receiver.
           If the sender has enough balance, it update the balances and return true or
           if the sender has insufficient balance, it return false. 
           It prints the appropriate message in each case. */

        let sender_balance = self.balances.entry(sender.clone()).or_insert(0);
        if *sender_balance < amount {
            println!("Transaction Failed: Insufficient funds for {}", sender);
            return false;
        }
        *sender_balance -= amount;
        let receiver_clone = receiver.clone(); //Clone receiver before moving it.
        let receiver_balance = self.balances.entry(receiver_clone).or_insert(0);

        *receiver_balance += amount;

        println!("\nTransaction Successful: {} sent {} to {}", sender, amount, receiver);
        true
    }

    pub fn show_balances(&self) {

        println!("Balances: {:?}", self.balances);

        /* To print the detailed Blockchain Ledger with Hash's info
        
        println!("\n===Blockchain Ledger ===");

        for block in &self.chain {
            println!("\n[Block {}] Nounce: {}\nPrevious Hash: {}\nCurrent Hash: {}\nData: {}\n", block.index, block.nonce, block.previous_hash, block.hash, block.data);
        }

        println!("=======================================\n");
        */
    }
}


