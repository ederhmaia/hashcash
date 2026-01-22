use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;

pub struct Chat {
    pub timestamp: String,
    pub message: String,
}

pub struct HashedChat {
    pub chat: Chat,
    pub hash: String,
    pub nonce: u64,
}

fn check_challenge(hash: &str, difficulty: u8) -> bool {
    hash.chars()
        .take(difficulty as usize)
        .all(|c| c == '0')
}

fn solve_challenge(chat: &Chat, difficulty: u8) -> (String, u64) {
    let mut nonce = 0u64;

    loop {
        let hash_bytes = Sha256::new()
            .chain_update(chat.timestamp.as_bytes())
            .chain_update(chat.message.as_bytes())
            .chain_update(nonce.to_string().as_bytes())
            .finalize();

        let hash = hex::encode(hash_bytes);

        if check_challenge(&hash, difficulty) {
            println!("Found solution: {} | Generated Hash: {}", nonce, hash);
            return (hash, nonce);
        }

        println!("Nonce failed: {} | Generated Hash: {}", nonce, hash);
        nonce += 1;
    }
}

// timestamp|message|nonce
pub fn send_chat(message: &str) -> HashedChat {
   let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let chat = Chat {
        timestamp: timestamp.to_string(),
        message: message.to_string(),
    };

    let DIFFICULTY = 5;

    let (hash, nonce) = solve_challenge(&chat, DIFFICULTY);

    HashedChat { chat, hash, nonce }
}
