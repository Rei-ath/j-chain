use crate::bc::block::Block;
use crate::mote::vote::Vote;
use crate::mote::voter::Voter;
use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{hash_set, HashMap, HashSet};

// #[derive(Debug)]
// pub enum Val {
//     Voter,
//     Candidate,
// }

// Define the blockchain itself
#[allow(dead_code)]
#[derive(Debug)]
pub struct Blockchain {
    pub(crate) voters: HashMap<String, Voter>,
    pub(crate) chain: Vec<Block>,
    pub(crate) candidates: Vec<String>,      // List of candidates
    pub(crate) votes: HashMap<String, Vote>, // Map to store votes for each candidate
    pub(crate) difficulty: usize,            // Difficulty for mining
    pub(crate) registered_users: HashSet<String>, // Set of registered users
}

impl Blockchain {
    // Function to create a new blockchain with specified difficulty
    /// Creates a new blockchain with the specified difficulty level.
    ///
    /// # Arguments
    ///
    /// * `difficulty` - The difficulty level for mining new blocks.
    ///
    /// # Returns
    ///
    /// A new `Blockchain` instance.
    pub fn new(difficulty: usize) -> Self {
        let data = json!({
            "age":0
        });
        let voter = Voter {
            id: 0,
            name: "rei".to_string(),
            data,
            vote_given: false,
            password: "idkidkidk".to_string(),
        };
        let genesis_block = Block::new(
            0,
            "genesis block".to_string(),
            Vote {
                candidate_name: "null".to_string(),
                voter: voter.clone(),
                signature: "null".to_string(),
            },
            '0'.to_string(),
            0,
        );
        let mut register_users = HashSet::new();
        register_users.insert("rei".to_string());
        let mut users = HashMap::new();
        users.insert(voter.name.clone(), voter);
        Self {
            chain: vec![genesis_block],
            candidates: Vec::new(),
            votes: HashMap::new(),
            difficulty,
            registered_users: HashSet::new(),
            voters: users,
        }
    }

    // Function to mine a new block
    /// Mines a new block in the blockchain.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the voter.
    /// * `vote` - The vote cast by the voter.
    pub fn mine_block(&mut self, name: String, vote: Vote) {
        let prev_hash = self.chain.last().unwrap().get_previous_hash();
        let index = self.chain.len() as u32;
        let now = Utc::now();
        let timestamp = now.timestamp() * 1000 + now.timestamp_subsec_millis() as i64;
        let mut nonce = 0;
        let mut hasher = Sha256::new();

        loop {
            hasher.update(format!("{}{}{}{}", index, timestamp, &prev_hash, nonce).as_bytes());
            let hash = format!("{:x}", hasher.finalize_reset());
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                let new_block = Block::create(
                    index,
                    timestamp.try_into().unwrap(),
                    name,
                    vote.clone(),
                    prev_hash.to_string(),
                    hash.clone(),
                    nonce,
                );
                self.chain.push(new_block);
                self.votes.insert(hash, vote.clone());
                break;
            }
            nonce += 1;
        }
    }


    pub fn display_votes(&self) {
        for (candidate, vote) in &self.votes {
            println!("Candidate {}: {:?}", candidate, vote);
        }
    }
    /// Registers a new user in the blockchain network.
    ///
    /// # Arguments
    ///
    /// * `k` - The key (ID) of the user.
    /// * `v` - The voter information.
    pub fn register_user(&mut self, k: String, v: Voter) {
        self.voters.insert(k.clone(), v); 
        // self.add_registered_user(k);
        // Clone the key before insertion
    }

    /// Adds a registered user to the set of registered users.
    ///
    /// # Arguments
    ///
    /// * `user` - The name of the user to be added.
    pub fn add_registered_user(&mut self, user: String) {
        self.registered_users.insert(user.clone().to_string()); // Clone the user before insertion
    }
}
