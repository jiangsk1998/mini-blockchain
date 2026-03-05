use std::fmt::{ Display, Formatter};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug,Clone,Serialize,Deserialize)]
struct Block{
    index: u64,  //区块高度
    timestamp: u64, //创建时间戳
    data:String, //区块数据
    prev_hash:String, //前一个区块的哈希
    nonce:u64, // 一次性数字
    hash:String, //本区快的哈希

}

// 区块hash的计算

impl Block{
    fn calculate_hash(index:u64,timestamp:u64,data:&str,prev_hash:&str)->String{
        let input = format!("{}{}{}{}",index,timestamp,data,prev_hash);
        let hash = Sha256::digest(input.as_bytes());
        hex::encode(hash)
    }
}

//创世区块 Genesis Block

impl Block{

    //创建创世区块
    fn genesis (difficulty: usize) -> Self{

        let index = 0;
        let nonce = 0;
        let timestamp = Self::current_timestamp();
        let data = "创世区块 - Genesis Block".to_string();
        let prev_hash = "0".repeat(64);

        let mut block = Block{index,timestamp, data,prev_hash,hash: String::new(),nonce};

        if difficulty > 0 {
            Self::mine_block(&mut block, difficulty);
        } else {
            block.hash = Self::calculate_hash_with_nonce(
                block.index, block.timestamp, &block.data, &block.prev_hash, block.nonce,
            );
        }

        block

    }

    fn mine_block(&mut self,difficulty:usize){
        let target = "0".repeat(difficulty);



        loop {
            // 计算哈希
           let hash = Self::calculate_hash_with_nonce(self.index,self.timestamp,&self.data,&self.prev_hash,self.nonce);
            // 校验hash是否满足难度要求
            if hash.starts_with(&target){
                self.hash = hash;
                println!("挖矿成功！ mine block hash: {},nocd {}", self.hash,self.nonce);
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

    fn calculate_hash_with_nonce(index: u64, timestamp: u64, data: &str, prev_hash: &str, nonce: u64) -> String {
        let input = format!("{}{}{}{}{}",index,timestamp,data,prev_hash,nonce);
        let hash = Sha256::digest(input.as_bytes());
        hex::encode(hash)
    }
}

// 创建新区块

impl Block{
    ///基于前一个区块创建新区快
    /// 参数： 前一个区块的引用，新区块的数据
    /// 返回： 新区块
    fn new (prev_block:&Block,data:String,difficulty: usize)->Self{

        //新区块高度
        let index = prev_block.index+1;
        let nonce = 0;
        // 新区块时间戳
        let timestamp = Self::current_timestamp();
        // 前一个区块的hash
        let prev_hash = prev_block.hash.clone();

        let mut block = Block{index,timestamp, data,prev_hash,hash: String::new(),nonce};

        if difficulty > 0 {
            Self::mine_block(&mut block, difficulty);
        } else {
            block.hash = Self::calculate_hash_with_nonce(
                block.index, block.timestamp, &block.data, &block.prev_hash, block.nonce,
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
        write!(f,"交易数据：{} \n",self.data)
    }
}

//区块链结构体

#[derive(Debug,Clone,Serialize,Deserialize)]
struct BlockChain{
    chain: Vec<Block>, //用动态数组存储所有区块
    difficulty: usize, //挖矿难度
}

impl BlockChain{
    ///创建新链
    fn new(difficulty: usize)->Self{
        BlockChain{
            // vec! 宏创建包含创世区块的数组
            chain:vec![Block::genesis(difficulty)],
            difficulty,
        }
    }

    ///添加新区块
    fn add_block(&mut self,data:String){
        //获取最后一个区块

        // .unwrap() 取出 &Block（这里不会失败，因为至少有创世区块）
        let prev_block = self.chain.last().unwrap();

        //基于前一个区块创建新区块
        let block = Block::new(prev_block,data,self.difficulty);

        //将新区块追加到链尾
        self.chain.push(block);
    }

    ///获取最新区块
    fn latest_block(&self)->&Block{
        self.chain.last().unwrap()
    }


    ///打印整条链
    fn print_chain(&self){
        println!("Chain:========区块链状态========");

        for block in &self.chain{
            // println!("区块：{}",block.index);
            // println!("时间戳：{}",block.timestamp);
            // println!("数据：{}",&block.data);
            // println!("工作量：{}",&block.nonce);
            // println!("前哈希：{}..{}",&block.prev_hash[..8], &block.prev_hash[56..]);
            // println!("本哈希：{}..{}",&block.hash[..8], &block.hash[56..]);
            // println!()

            println!("{}", block);
        }
    }
}

//链的校验  防篡改核心

impl BlockChain{
    fn is_valid(&self) -> bool{
        // 从第一个区块开始遍历 跳过创世区块

        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let prev_block = &self.chain[i-1];

            //检查1：当前区块的hash是否正确

            let recalaulat_hash = Block::calculate_hash_with_nonce(current_block.index,current_block.timestamp,&current_block.data,&current_block.prev_hash,current_block.nonce);
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

impl BlockChain{
    fn save2file(&self,path:&str)->Result<(), Box<dyn std::error::Error>>{
        let json = serde_json::to_string(self)?;
        fs::write(path,json)?;
        Ok(())
    }

    fn load_from_file(path:&str)->Result<BlockChain,Box<dyn std::error::Error>>{
        let json = fs::read_to_string(path)?;
        let blockchain = serde_json::from_str(&json)?;
        Ok(blockchain)
    }
}

impl BlockChain {

    fn block_count(&self)->usize{
        self.chain.len()
    }

    fn total_data_size(&self)->usize{
        self.chain.iter().fold(0,|acc,x| acc + x.data.len())
    }

}


fn main() {
    println!("=== Step 1: 创建区块链 ===\n");

    let mut block_chain = BlockChain::new(2);

    println!("=== Step 2: 添加区块 ===\n");

    block_chain.add_block("张三 转 100 给 李四".to_string());
    block_chain.add_block("王五 转 100 给 李四".to_string());
    block_chain.add_block("李四 转 100 给 赵六".to_string());

    BlockChain::save2file(&block_chain, "block_chain.json").unwrap();

    block_chain = BlockChain::load_from_file("block_chain.json").unwrap();

    block_chain.print_chain();

    println!("区块高度：{}",block_chain.block_count());
    println!("区块数据大小：{}",block_chain.total_data_size());

    println!("\n=== Step 3: 验证链完整性 ===");

    println!("校验链的完整性：{}", block_chain.is_valid());

    println!("=== Step 4: 模拟篡改攻击 ===");
    println!("篡改区块 #1 的数据...");



    block_chain.chain[1].data = "张三 转 100 给 小P".to_string();

    println!("篡改后验证: {}", block_chain.is_valid());
}
