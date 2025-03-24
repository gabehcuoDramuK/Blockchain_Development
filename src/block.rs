use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub data: String,
    pub nonce: i32,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        data: String,
        difficulty: usize,
        timestamp: u64,
    ) -> Self {
        // TODO: Create a new block with the provided data and mine it with the given difficulty
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            data,
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.mine(difficulty);
        block
    }

    pub fn calculate_hash(&self) -> String {
        // TODO: calculate the hash of the block by concatenating the block fields
        // index, timestamp, previous_hash, data, and nonce and hashing them
        // using the SHA-256 algorithm
        let input = format!("{}{}{}{}{}", self.index, self.timestamp, self.previous_hash, self.data, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) -> String {
        let target = "0".repeat(difficulty);
        loop {
            let hash = self.calculate_hash();

            if hash.starts_with(&target) {
                return hash;
            }
            self.nonce += 1;
        }
    }
}
