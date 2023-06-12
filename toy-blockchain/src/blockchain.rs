use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

use chrono::prelude::*;

#[derive(Serialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>
}

#[derive(Serialize, Debug)]
pub struct Chain {
    chain: Vec<Block>,
    mempool: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: f32
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let Chain1 = Chain {
            chain: Vec::new(),
            mempool: Vec::new(),
            difficulty,
            miner_address,
            reward: 100.0
        };
        Chain1
    }

    pub fn new_tx(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.mempool.push(Transaction{
            sender,
            receiver,
            amount
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn mine_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_tx = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header: header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_tx);
        block.transactions.append(&mut self.mempool);
        self.mempool.clear();
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for item in vec_res {
            write!(&mut s, "{:x}", item).expect("unable to write");
        }
        s
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if (val != 0) {
                        header.nonce += 1;
                    } else {
                        println!("Block hash is: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    break;
                }
            };
        }
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for i in curr_trans {
            let hash = Chain::hash(&i);
            merkle.push(hash);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);            
            let mut h2 = merkle.remove(0);            
            h1.push_str(&mut h2);
            let newh = Chain::hash(&h1);
            merkle.push(newh);
        }

        return merkle.remove(0);
    }
}