use std::fs;
use serde::{Deserialize, Serialize};
use crate::block::Block;  // 从 block 模块导入 Block

//区块链结构体

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct BlockChain{
    pub chain: Vec<Block>,   //用动态数组存储所有区块
    pub difficulty: usize,   //挖矿难度
}

impl BlockChain{
    ///创建新链
    pub fn new(difficulty: usize) -> Self{
        BlockChain{
            // vec! 宏创建包含创世区块的数组
            chain: vec![Block::genesis(difficulty)],
            difficulty,
        }
    }

    ///添加新区块
    pub fn add_block(&mut self, data: String){
        let prev_block = self.chain.last().unwrap();
        let block = Block::new(prev_block, data, self.difficulty);
        self.chain.push(block);
    }

    ///获取最新区块
    pub fn latest_block(&self) -> &Block{
        self.chain.last().unwrap()
    }

    ///打印整条链
    pub fn print_chain(&self){
        println!("Chain:========区块链状态========");
        for block in &self.chain{
            println!("{}", block);
        }
    }
}

//链的校验  防篡改核心

impl BlockChain{
    pub fn is_valid(&self) -> bool{
        // 从第一个区块开始遍历 跳过创世区块
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let prev_block = &self.chain[i-1];

            //检查1：当前区块的hash是否正确
            let recalaulat_hash = Block::calculate_hash_with_nonce(
                current_block.index,
                current_block.timestamp,
                &current_block.data,
                &current_block.prev_hash,
                current_block.nonce
            );
            if recalaulat_hash != current_block.hash {
                println!("区块 #{} 哈希不匹配！数据已被篡改", current_block.index);
                return false;
            }
            //检查2：prev_hash是否指向前一个区块的hash
            if current_block.prev_hash != prev_block.hash {
                println!("区块 #{} 的链接断裂！prev_hash 不匹配", current_block.index);
                return false;
            }
        }
        //所有区块都通过
        true
    }
}

// 文件持久化

impl BlockChain{
    pub fn save2file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>{
        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<BlockChain, Box<dyn std::error::Error>>{
        let json = fs::read_to_string(path)?;
        let blockchain = serde_json::from_str(&json)?;
        Ok(blockchain)
    }
}

// 统计信息

impl BlockChain {
    pub fn block_count(&self) -> usize{
        self.chain.len()
    }

    pub fn total_data_size(&self) -> usize{
        self.chain.iter().fold(0, |acc, x| acc + x.data.len())
    }
}
