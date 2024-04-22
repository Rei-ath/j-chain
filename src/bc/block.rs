use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::mote::vote::Vote;

/// Represents a block in the blockchain.
#[derive(Clone, Debug)]
pub(crate) struct Block {
    /// The index of the block in the blockchain.
    index: u32,
    /// The name associated with the block.
    name: String,
    /// The timestamp indicating when the block was created.
    timestamp: u64,
    /// The vote stored in the block.
    vote: Vote,
    /// The hash of the previous block in the blockchain.
    prev_hash: String,
    /// The hash of the current block.
    hash: String,
    /// The nonce used in mining the block.
    nonce: u64,
}

impl Block {
    /// Creates a new block in the blockchain.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the block.
    /// * `name` - The name associated with the block.
    /// * `vote` - The vote stored in the block.
    /// * `prev_hash` - The hash of the previous block.
    /// * `nonce` - The nonce used in mining the block.
    ///
    /// # Returns
    ///
    /// A new `Block` instance.
    pub fn new(index: u32, name: String, vote: Vote, prev_hash: String, nonce: u64) -> Self {
        let timestamp: u64 = Utc::now().timestamp() as u64;
        let hash = Block::calculate_hash(index, timestamp, &name, &prev_hash);

        Block {
            index,
            name,
            timestamp,
            vote,
            prev_hash,
            hash,
            nonce,
        }
    }

    /// Creates a block with provided data.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the block.
    /// * `timestamp` - The timestamp indicating when the block was created.
    /// * `name` - The name associated with the block.
    /// * `vote` - The vote stored in the block.
    /// * `prev_hash` - The hash of the previous block.
    /// * `hash` - The hash of the current block.
    /// * `nonce` - The nonce used in mining the block.
    ///
    /// # Returns
    ///
    /// A new `Block` instance.
    pub fn create(
        index: u32,
        timestamp: u64,
        name: String,
        vote: Vote,
        prev_hash: String,
        hash: String,
        nonce: u64,
    ) -> Self {
        Block {
            index,
            name,
            timestamp,
            vote,
            prev_hash,
            hash,
            nonce,
        }
    }

    /// Calculates the hash of a block.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the block.
    /// * `timestamp` - The timestamp indicating when the block was created.
    /// * `data` - The data stored in the block.
    /// * `previous_hash` - The hash of the previous block.
    ///
    /// # Returns
    ///
    /// The calculated hash as a hexadecimal string.
    pub fn calculate_hash(index: u32, timestamp: u64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Returns the name associated with the block.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Returns the index of the block.
    pub fn get_index(&self) -> u32 {
        self.index
    }

    /// Returns the timestamp indicating when the block was created.
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Returns a reference to the vote stored in the block.
    pub fn get_vote(&self) -> &Vote {
        &self.vote
    }

    /// Returns the hash of the previous block.
    pub fn get_previous_hash(&self) -> &str {
        &self.prev_hash
    }

    /// Returns the hash of the current block.
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    /// Returns the nonce used in mining the block.
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}
