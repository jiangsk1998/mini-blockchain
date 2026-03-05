mod block;       // 声明 block 模块 → 对应 src/block.rs
mod blockchain;  // 声明 blockchain 模块 → 对应 src/blockchain.rs

use blockchain::BlockChain;  // 从 blockchain 模块导入 BlockChain

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
