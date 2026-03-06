use std::fmt::{Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block{
    pub transactions: Vec<Transaction>,
    pub index: u64,        //区块高度
    pub timestamp: u64,    //创建时间戳
    // pub data: String,      //区块数据  从data字段升级到transactions列表
    pub prev_hash: String, //前一个区块的哈希
    pub nonce: u64,        //一次性数字
    pub hash: String,      //本区块的哈希
}

// 区块hash的计算

impl Block{
    pub fn calculate_hash_with_nonce(index: u64, timestamp: u64, transactions:  &[Transaction], prev_hash: &str, nonce: u64) -> String {
        let tx_json = serde_json::to_string(&transactions).unwrap();
        let input = format!("{}{}{}{}{}",index,timestamp,tx_json,prev_hash,nonce);
        let hash = Sha256::digest(input.as_bytes());
        hex::encode(hash)
    }
}

//创世区块 Genesis Block

impl Block{

    //创建创世区块
    pub fn genesis(difficulty: usize) -> Self{
        let index = 0;
        let nonce = 0;
        let timestamp = Self::current_timestamp();
        // let data = "创世区块 - Genesis Block".to_string();
        let transactions = Vec::new();
        let prev_hash = "0".repeat(64);

        let mut block = Block{index,timestamp, transactions,prev_hash,hash: String::new(),nonce};

        if difficulty > 0 {
            Self::mine_block(&mut block, difficulty);
        } else {
            block.hash = Self::calculate_hash_with_nonce(
                block.index, block.timestamp, &block.transactions, &block.prev_hash, block.nonce,
            );
        }

        block
    }

    fn mine_block(&mut self, difficulty: usize){
        let target = "0".repeat(difficulty);

        loop {
            // 计算哈希
            let hash = Self::calculate_hash_with_nonce(self.index,self.timestamp,&self.transactions,&self.prev_hash,self.nonce);
            // 校验hash是否满足难度要求
            if hash.starts_with(&target){
                self.hash = hash;
                println!("挖矿成功！ mine block hash: {},nonce {}", self.hash,self.nonce);
                break;
            }

            // 没找到 继续挖 nonce +1
            self.nonce += 1;
        }
    }

    //获取当前时间戳 秒
    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

// 创建新区块

impl Block{
    ///基于前一个区块创建新区块
    /// 参数： 前一个区块的引用，新区块的数据
    /// 返回： 新区块
    pub fn new(prev_block: &Block, transactions: Vec<Transaction>, difficulty: usize) -> Self{
        //新区块高度
        let index = prev_block.index+1;
        let nonce = 0;
        // 新区块时间戳
        let timestamp = Self::current_timestamp();
        // 前一个区块的hash
        let prev_hash = prev_block.hash.clone();

        let mut block = Block{index,timestamp, transactions,prev_hash,hash: String::new(),nonce};

        if difficulty > 0 {
            Self::mine_block(&mut block, difficulty);
        } else {
            block.hash = Self::calculate_hash_with_nonce(
                block.index, block.timestamp, &block.transactions, &block.prev_hash, block.nonce,
            );
        }

        block
    }
}

impl Display for Block{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"=====区块高度====：{} \n",self.index)?;
        write!(f,"本区块哈希：{} \n",self.hash)?;
        write!(f,"前区块哈希：{} \n",self.prev_hash)?;
        write!(f,"时间戳：{} \n",self.timestamp)?;
        write!(f,"nonce：{} \n",self.nonce)?;
        write!(f,"交易数据：{:?} \n",self.transactions)
    }
}
