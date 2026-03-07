
mod blockchain;
mod crypto;
// 声明 blockchain 模块 → 对应 src/blockchain.rs

use std::fmt::Display;
// use crate::block::Block;
use crate::blockchain::{BlockChain, Transaction};

// 从 blockchain 模块导入 BlockChain

// fn main() {
//     println!("=== Step 1: 创建区块链 ===\n");
//
//     let mut block_chain = BlockChain::new(2);
//
//     println!("=== Step 2: 添加区块 ===\n");
//
//     block_chain.add_block("张三 转 100 给 李四".to_string());
//     block_chain.add_block("王五 转 100 给 李四".to_string());
//     block_chain.add_block("李四 转 100 给 赵六".to_string());
//
//     BlockChain::save2file(&block_chain, "block_chain.json").unwrap();
//
//     block_chain = BlockChain::load_from_file("block_chain.json").unwrap();
//
//     block_chain.print_chain();
//
//     println!("区块高度：{}",block_chain.block_count());
//     println!("区块数据大小：{}",block_chain.total_data_size());
//
//     println!("\n=== Step 3: 验证链完整性 ===");
//
//     println!("校验链的完整性：{}", block_chain.is_valid());
//
//     println!("=== Step 4: 模拟篡改攻击 ===");
//     println!("篡改区块 #1 的数据...");
//
//     block_chain.chain[1].data = "张三 转 100 给 小P".to_string();
//
//     println!("篡改后验证: {}", block_chain.is_valid());
// }

///区块链交易流程：用户发起交易 → 签名 → 加入交易池 → 验证（签名+余额）
///              → 矿工打包 → 挖矿 → 新区块 → 更新余额 → 加入链
fn main() {
    println!("=== Step 1: 创建区块链 ===\n");
    let mut bc = BlockChain::new(2); // 难度为 2（哈希前 2 位必须是 0）

    println!("=== Step 2: 创建账户 ===\n");
    // 生成三个账户的密钥对
    // SigningKey::generate(&mut OsRng) → 用密码学安全随机数生成 Ed25519 私钥
    // &mut OsRng → OsRng 的可变引用（生成随机数会修改内部状态，所以需要 &mut）
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);
    let charlie_key = SigningKey::generate(&mut OsRng);
    let miner_key = SigningKey::generate(&mut OsRng);

    // 链式调用拆解：
    //   alice_key.verifying_key()        → 从私钥推导出公钥（VerifyingKey）
    //                    .as_bytes()     → 将公钥转为 &[u8; 32] 字节数组引用
    //   hex::encode(...)                 → 将字节数组转为十六进制字符串，作为地址
    let alice_addr = hex::encode(alice_key.verifying_key().as_bytes());
    let bob_addr = hex::encode(bob_key.verifying_key().as_bytes());
    let charlie_addr = hex::encode(charlie_key.verifying_key().as_bytes());
    let miner_addr = hex::encode(miner_key.verifying_key().as_bytes());

    // 初始化余额
    bc.accounts.set_balance(&alice_addr, 100000);
    bc.accounts.set_balance(&bob_addr, 50000);
    bc.accounts.set_balance(&charlie_addr, 0);
    bc.accounts.set_balance(&miner_addr, 0);

    // &alice_addr[..16] → 字符串切片，取前 16 个字符（地址太长，只显示一部分）
    println!("Alice:   {} (余额: 1000)", &alice_addr[..16]);
    println!("Bob:     {} (余额: 500)", &bob_addr[..16]);
    println!("Charlie: {} (余额: 0)\n", &charlie_addr[..16]);
    println!("矿工余额: {} (余额: 0)\n", &miner_addr[..16]);

    println!("=== Step 3: 发起交易 ===\n");
    // Transaction::new() → 创建交易并自动用私钥签名
    let tx1 = Transaction::new(&alice_key, &bob_addr, 10000);

    // 链式调用拆解：
    //   bc.add_transaction(tx1)          → 验证交易（签名+余额+金额）并加入交易池
    //                                      返回 Result<(), String>
    //                     .unwrap()      → 取出 Ok 的值（如果是 Err 会 panic）
    //                                      这里我们确信交易有效，所以用 unwrap
    bc.add_transaction(tx1).unwrap();
    println!("✓ Alice → Bob: 100");

    let tx2 = Transaction::new(&bob_key, &charlie_addr, 5000);
    bc.add_transaction(tx2).unwrap();
    println!("✓ Bob → Charlie: 50\n");

    println!("=== Step 4: 挖矿打包交易 ===\n");
    // mine_pending_transactions() → 将交易池中的交易打包到新区块
    bc.mine_pending_transactions(&miner_addr);

    println!("\n=== Step 5: 查看余额 ===\n");
    // bc.accounts.get_balance(&addr) → 获取指定地址的余额
    println!("Alice:   {}", bc.accounts.get_balance(&alice_addr)); // 899
    println!("Bob:     {}", bc.accounts.get_balance(&bob_addr)); // 549
    println!("Charlie: {}", bc.accounts.get_balance(&charlie_addr)); // 50
    println!("miner: {}", bc.accounts.get_balance(&miner_addr)); // 1.5

    println!("\n=== Step 6: 验证链完整性 ===\n");
    // bc.is_valid() → 验证整条链的哈希和签名是否正确
    println!("链是否有效: {}", bc.is_valid());

    println!("\n=== Step 7: 尝试余额不足的交易 ===\n");
    // Charlie 只有 50，尝试转 100 → 应该失败
    let tx3 = Transaction::new(&charlie_key, &alice_addr, 10000);
    // match → 模式匹配，分别处理成功（Ok）和失败（Err）两种情况
    // Java 类比：try { ... } catch (Exception e) { ... }
    match bc.add_transaction(tx3) {
        Ok(_) => println!("交易成功"), // _ 表示忽略 Ok 里的值（是空的 ()）
        Err(e) => println!("交易失败: {}", e), // e 是错误信息 String
    }

    let history = bc.get_transaction_history(&alice_addr);

    for tx in &history {
        println!("from: {} -> to: {}, amount: {}, fee: {}",
                 &tx.from[..16], &tx.to[..16], tx.amount, tx.fee);
    }
}


