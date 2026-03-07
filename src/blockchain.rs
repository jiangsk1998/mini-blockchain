use crate::account::AccountManager;
use crate::block::Block;
use crate::transaction::Transaction;
use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
// 从 block 模块导入 Block

//区块链结构体

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockChain {
    pub chain: Vec<Block>,                      //用动态数组存储所有区块
    pub difficulty: usize,                      //挖矿难度
    pub pending_transactions: Vec<Transaction>, //待打包的交易池
    pub accounts: AccountManager,
    pub used_tx_hashes: HashSet<String>, //已用哈希集合
}

impl BlockChain {
    ///创建新链
    pub fn new(difficulty: usize) -> Self {
        BlockChain {
            // vec! 宏创建包含创世区块的数组
            chain: vec![Block::genesis(difficulty)],
            difficulty,
            pending_transactions: vec![],
            accounts: AccountManager::new(),
            used_tx_hashes: HashSet::new(),
        }
    }

    ///添加新区块->添加交易到交易池
    pub fn add_block(&mut self, transactions: Vec<Transaction>, miner: &str) {
        let prev_block = self.latest_block();
        let block = Block::new(prev_block, transactions, self.difficulty, miner.to_string());
        self.chain.push(block);
    }

    ///获取最新区块
    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    ///打印整条链
    pub fn print_chain(&self) {
        println!("Chain:========区块链状态========");
        for block in &self.chain {
            println!("{}", block);
        }
    }
}

//链的校验  防篡改核心

impl BlockChain {
    // pub fn is_valid(&self) -> bool{
    //     // 从第一个区块开始遍历 跳过创世区块
    //     for i in 1..self.chain.len() {
    //         let current_block = &self.chain[i];
    //         let prev_block = &self.chain[i-1];
    //
    //         //检查1：当前区块的hash是否正确
    //         let recalaulat_hash = Block::calculate_hash_with_nonce(
    //             current_block.index,
    //             current_block.timestamp,
    //             &current_block.transactions,
    //             &current_block.prev_hash,
    //             current_block.nonce
    //         );
    //         if recalaulat_hash != current_block.hash {
    //             println!("区块 #{} 哈希不匹配！数据已被篡改", current_block.index);
    //             return false;
    //         }
    //         //检查2：prev_hash是否指向前一个区块的hash
    //         if current_block.prev_hash != prev_block.hash {
    //             println!("区块 #{} 的链接断裂！prev_hash 不匹配", current_block.index);
    //             return false;
    //         }
    //     }
    //     //所有区块都通过
    //     true
    // }
}

// 文件持久化

impl BlockChain {
    pub fn save2file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<BlockChain, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let blockchain = serde_json::from_str(&json)?;
        Ok(blockchain)
    }
}

// 统计信息

impl BlockChain {
    pub fn block_count(&self) -> usize {
        self.chain.len()
    }

    pub fn total_data_size(&self) -> usize {
        self.chain
            .iter()
            .fold(0, |acc, x| acc + x.transactions.len())
    }
}

///区块链升级  交易池+验证
impl BlockChain {
    ///添加交易到交易池(带验证)
    pub(crate) fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        //交易入池检查

        if self.used_tx_hashes.contains(&transaction.hash) {
            return Err("不可重复交易".to_string());
        }

        //3.金额是否>0
        if transaction.amount == 0 {
            return Err("转账金额不能为0".to_string());
        }
        //2.余额是否足够
        // if &self.accounts.get_balance(&transaction.from) < &transaction.amount {
        //     return Err("余额不足！".to_string());
        // }
        //已经实现了验证余额的方法
        if !self
            .accounts
            .has_sufficient_balance(&transaction.from, transaction.amount + transaction.fee)
        {
            return Err("余额不足！".to_string());
        }

        //1.验证签名是否有效
        if !transaction.verify_signature() {
            return Err("签名无效!".to_string());
        }
        //4.验证通过 加入交易池
        self.pending_transactions.push(transaction);
        Ok(())
    }
    ///挖矿：打包交易池中的交易到新区块
    pub(crate) fn mine_pending_transactions(&mut self, miner: &str) {
        //1.创建新区块 包含所有待处理交易  通过add_block实现
        // let prev_block = self.latest_block();
        // let block = Block::new(
        //     prev_block,
        //     self.pending_transactions.clone(),
        //     self.difficulty,
        //     miner.clone().to_string()
        // );

        // 1.添加coinbase

        let coinbase = Transaction {
            from: "coinbase".to_string(),
            to: miner.to_string(),
            amount: 50,
            timestamp: Transaction::current_timestamp(),
            fee: 0,
            signature: Signature::from_bytes(&[0u8; 64]),
            hash: "0".repeat(64).to_string(),
        };
        &self.pending_transactions.insert(0, coinbase);

        //2.执行所有交易（更新余额）
        for x in &self.pending_transactions {
            //coinbase不是转账
            //如果是coinbase
            if x.from == "coinbase" {
                let miner_balance = self.accounts.get_balance(&miner);
                self.accounts
                    .set_balance(&miner.to_string(), miner_balance + x.amount);
                continue
            } else {
                self.accounts
                    .transfer(&x.from, &x.to, x.amount, x.fee, miner);
            }
            //上链登记
            self.used_tx_hashes.insert(x.hash.clone());
        }
        //3.将新区块加入链
        self.add_block(self.pending_transactions.clone(), miner);

        //4.清空交易池
        self.pending_transactions.clear();

        println!("{}", "新区块已挖出！")
    }

    ///验证整条链
    pub(crate) fn is_valid(&self) -> bool {
        //1.从第一个区块遍历到最后  跳过创世区块 0

        for i in 1..self.chain.len() {
            //检查：1 区块哈希是否正确
            let current_block = &self.chain[i];
            if !current_block.hash.eq(&Block::calculate_hash_with_nonce(
                current_block.index,
                current_block.timestamp,
                &current_block.transactions,
                &current_block.prev_hash.to_string(),
                current_block.nonce,
                &current_block.miner,
            )) {
                return false;
            }

            //检查：2 链接是否正确 prev_hash指向前一个区块的hash
            if !current_block.prev_hash.eq(&self.chain[i - 1].hash) {
                return false;
            }

            //检查：3 所有交易签名是否有效
            for (j, tx) in current_block.transactions.iter().enumerate() {
                // CoinBase不需要验签
                if tx.from.eq("coinbase") {
                    continue;
                }

                //list带索引迭代器
                if !tx.verify_signature() {
                    println!("区块 {} 的第 {} 笔交易签名无效", current_block.index, j);
                    return false;
                }
            }
        }
        true
    }
}

impl BlockChain {
    pub fn get_transaction_history(&self, address: &str) -> Vec<&Transaction> {
        let mut trans_history = Vec::new();

        for x in self.chain.iter() {
            for j in x.transactions.iter() {
                if j.from.eq(address) || j.to.eq(address) {
                    trans_history.push(j)
                }
            }
        }
        trans_history
    }
}
