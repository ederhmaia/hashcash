use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub timestamp: String,
    pub message: String,
    pub sender: String,
}

impl Chat {
    pub fn new(message: String, sender: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        
        Self { timestamp, message, sender }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashedChat {
    pub chat: Chat,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u8,
}

impl HashedChat {
    fn calculate_hash(timestamp: &str, message: &str, sender: &str, nonce: u64) -> String {
        let hash_bytes = Sha256::new()
            .chain_update(timestamp.as_bytes())
            .chain_update(message.as_bytes())
            .chain_update(sender.as_bytes())
            .chain_update(nonce.to_string().as_bytes())
            .finalize();
        hex::encode(hash_bytes)
    }

    pub fn verify(&self) -> bool {
        let hash = Self::calculate_hash(
            &self.chat.timestamp,
            &self.chat.message,
            &self.chat.sender,
            self.nonce
        );
        
        hash == self.hash && has_leading_zeros(&hash, self.difficulty)
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }
}

fn has_leading_zeros(hash: &str, count: u8) -> bool {
    hash.chars()
        .take(count as usize)
        .all(|c| c == '0')
}

fn solve_challenge(chat: &Chat, difficulty: u8) -> (String, u64) {
    let mut nonce = 0u64;
    loop {
        let hash = HashedChat::calculate_hash(
            &chat.timestamp,
            &chat.message,
            &chat.sender,
            nonce
        );
        
        if has_leading_zeros(&hash, difficulty) {
            println!("Found solution: {} | Generated Hash: {}", nonce, hash);
            return (hash, nonce);
        }
        println!("Nonce failed: {} | Generated Hash: {}", nonce, hash);
        nonce += 1;
    }
}

pub fn send_chat(message: &str, sender: &str, difficulty: u8) -> HashedChat {
    let chat = Chat::new(message.to_string(), sender.to_string());
    let (hash, nonce) = solve_challenge(&chat, difficulty);
    
    HashedChat { 
        chat, 
        hash, 
        nonce,
        difficulty,
    }
}
