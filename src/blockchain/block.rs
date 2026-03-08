use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{Display, Formatter};
use std::io::repeat;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::blockchain::Transaction;
use crate::crypto::hash::sha256_hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {  //固定大小的元数据
    pub index: u64,     //区块高度
    pub timestamp: u64, //创建时间戳
    pub merkle: String, //merkle root
    pub prev_hash: String, //前一个区块的哈希
    pub nonce: u64,        //一次性数字
    pub miner: String,    //矿工地址
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBody {
    pub transactions: Vec<Transaction>, //交易信息
    pub tx_count: u64, //交易数量
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_header: BlockHeader, //区块头
    pub block_body: BlockBody,
    pub hash: String,      //本区块的哈希

}

// 区块hash的计算

impl Block {
    pub(crate) fn calculate_hash(
        index: u64,
        timestamp: u64,
        merkle: &str,
        prev_hash: &str,
        nonce: u64,
        miner: &str,
    ) -> String {
        let input = format!("{}{}{}{}{}{}", index, timestamp, merkle, prev_hash, nonce, miner);
        // let hash = Sha256::digest(input.as_bytes());
        // hex::encode(hash)
        sha256_hash(input.as_bytes())
    }
}

//创世区块 Genesis Block

impl Block {
    //创建创世区块
    pub fn genesis(difficulty: usize) -> Self {
        let index = 0;
        let nonce = 0;
        let merkle = String::new();
        let timestamp = Self::current_timestamp();
        let prev_hash = "0".repeat(64);
        let miner = String::new();

        let block_header = BlockHeader {
            index,
            timestamp,
            merkle,
            prev_hash,
            nonce,
            miner,
        };

        let block_body = BlockBody {
            transactions: vec![],
            tx_count: 0,
        };

        let mut block = Block {
            block_header,
            block_body,
            hash: "0".repeat(64),
        };

        if difficulty > 0 {
            Self::mine_block(&mut block, difficulty);
        } else {
            block.hash = Self::calculate_hash(
                index,
                timestamp,
                &block.block_header.merkle,
                &block.block_header.prev_hash,
                block.block_header.nonce,
                &block.block_header.miner,
            );
        }

        block
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        loop {
            // 计算哈希
            let hash = Self::calculate_hash(
                self.block_header.index,
                self.block_header.timestamp,
                &self.block_header.merkle,
                &self.block_header.prev_hash,
                self.block_header.nonce,
                &self.block_header.miner,
            );
            // 校验hash是否满足难度要求
            if hash.starts_with(&target) {
                self.hash = hash;
                println!(
                    "挖矿成功！ mine block hash: {},nonce {}",
                    self.hash, self.block_header.nonce
                );
                break;
            }

            // 没找到 继续挖 nonce +1
            self.block_header.nonce += 1;
        }
    }

    //获取当前时间戳 秒
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Block {
    ///计算默克尔根
    pub fn compute_merkel_root(transactions: &Vec<Transaction>) -> String {
        //预校验
        if transactions.len() == 0 {
            return "0".repeat(64);
        }
        // 计算vec[hash]
        let mut tx_hashes: Vec<String> = transactions.iter().map(|tx| hex::encode(Sha256::digest(tx.hash.as_bytes()))).collect();
        // 如果只有一条交易 自己为根
        if tx_hashes.len() == 1 {
            return tx_hashes.remove(0);
        }

        while tx_hashes.len() > 1 {
            if tx_hashes.len() % 2 != 0 {
                //计数末尾补一个
                let last_hash = tx_hashes[tx_hashes.len() - 1].clone();
                tx_hashes.push(last_hash);
            }

            let mut next = Vec::new();

            for pair in tx_hashes.chunks(2) {
                let str = format!("{}{}", pair[0], pair[1]);
                next.push(hex::encode(Sha256::digest(str.as_bytes())))
            }
            tx_hashes = next;
        }
        tx_hashes.remove(0)
    }


}

// 创建新区块

impl Block {
    ///基于前一个区块创建新区块
    /// 参数： 前一个区块的引用，新区块的数据
    /// 返回： 新区块
    pub fn new(prev_block: &Block, transactions: Vec<Transaction>, difficulty: usize, miner: String) -> Self {
        //新区块高度
        let index = prev_block.block_header.index + 1;
        // 新区块时间戳
        let timestamp = Self::current_timestamp();
        // 前一个区块的hash
        let prev_hash = prev_block.hash.clone();

        let block_header = BlockHeader {
            index,
            timestamp,
            merkle: Self::compute_merkel_root(&transactions),
            prev_hash,
            nonce: 0,
            miner,
        };
        let tx_count = transactions.len() as u64;
        let block_body = BlockBody {
            transactions,
            tx_count,
        };

        let block_header_clone = block_header.clone();
        let mut block = Block {
            block_header,
            hash: Self::calculate_hash(
                block_header_clone.index,
                block_header_clone.timestamp,
                &block_header_clone.merkle,
                &block_header_clone.prev_hash,
                block_header_clone.nonce,
                &block_header_clone.miner,
            ),
            block_body,
        };

        if difficulty > 0 {
            Self::mine_block(&mut block, difficulty);
        } else {
            block.hash = Self::calculate_hash(
                block.block_header.index,
                block.block_header.timestamp,
                &block.block_header.merkle,
                &block.block_header.prev_hash,
                block.block_header.nonce,
                &block.block_header.miner,
            );
        }

        block
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "=====区块高度====：{} \n", self.block_header.index)?;
        write!(f, "本区块哈希：{} \n", self.hash)?;
        write!(f, "前区块哈希：{} \n", self.block_header.prev_hash)?;
        write!(f, "时间戳：{} \n", self.block_header.timestamp)?;
        write!(f, "nonce：{} \n", self.block_header.nonce)?;
        write!(f, "交易数据：{:?} \n", self.block_body.transactions)
    }
}
