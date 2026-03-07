# Mini Blockchain | 迷你区块链

[中文](#中文) | [English](#english)

---

## 中文

### 简介
一个用 Rust 实现的完整区块链项目，包含工作量证明（PoW）、交易系统、数字签名、账户管理等核心功能，用于学习区块链原理。

### 特性
- ✅ **工作量证明（PoW）** - 可配置难度的挖矿机制
- ✅ **交易系统** - 完整的交易验证和交易池
- ✅ **数字签名** - Ed25519 签名验证
- ✅ **账户管理** - 余额管理和转账功能
- ✅ **Coinbase 交易** - 矿工挖矿奖励机制
- ✅ **链验证** - 完整性验证和防篡改检测
- ✅ **防重放** - 交易哈希去重机制
- ✅ **交易历史** - 地址交易记录查询
- ✅ **数据持久化** - JSON 序列化存储

### 快速开始

#### 运行项目
```bash
cargo run
```

#### 构建项目
```bash
cargo build --release
```

### 核心概念

#### 交易流程
```
用户发起交易 → 私钥签名 → 加入交易池 → 验证（签名+余额+金额）
    ↓
矿工打包交易 → 挖矿（PoW）→ 新区块 → 更新余额 → 加入链
```

#### 区块结构
- `index`: 区块高度
- `timestamp`: 创建时间戳
- `transactions`: 交易列表
- `prev_hash`: 前一个区块的哈希
- `nonce`: 工作量证明的随机数
- `hash`: 当前区块的哈希
- `miner`: 矿工地址

#### 交易结构
- `from`: 发送方地址（公钥十六进制）
- `to`: 接收方地址
- `amount`: 转账金额
- `fee`: 手续费
- `timestamp`: 时间戳（防重放）
- `signature`: Ed25519 数字签名
- `hash`: 交易哈希

#### 工作量证明（PoW）
通过不断尝试不同的 nonce 值，找到满足难度要求的哈希（前导零数量）。

#### 难度配置
创建区块链时可指定难度：
```rust
let blockchain = BlockChain::new(2);  // 难度为 2（需要 2 个前导零）
```

#### Coinbase 交易
- 矿工挖矿的奖励机制
- 特殊标记：`from = "coinbase"`
- 凭空增发新币，不扣任何账户
- 跳过签名验证
- 放在区块第一笔交易

### 项目结构
```
mini-blockchain/
├── src/
│   ├── main.rs          # 主程序和演示代码
│   ├── block.rs         # 区块结构和挖矿逻辑
│   ├── blockchain.rs    # 区块链核心逻辑
│   ├── transaction.rs   # 交易和签名验证
│   └── account.rs       # 账户和余额管理
├── Cargo.toml           # 项目配置
├── README.md            # 说明文档
└── coinbase.md          # Coinbase 交易说明
```

### 技术栈
- **Rust** - 编程语言
- **sha2** - SHA-256 哈希算法
- **hex** - 十六进制编码
- **ed25519-dalek** - Ed25519 数字签名
- **serde** - 序列化/反序列化
- **rand** - 随机数生成

### 使用示例

#### 基本流程
```rust
use mini_blockchain::blockchain::BlockChain;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() {
    // 1. 创建区块链
    let mut bc = BlockChain::new(2);

    // 2. 生成账户密钥对
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);
    let miner_key = SigningKey::generate(&mut OsRng);

    // 3. 获取地址（公钥十六进制）
    let alice_addr = hex::encode(alice_key.verifying_key().as_bytes());
    let bob_addr = hex::encode(bob_key.verifying_key().as_bytes());
    let miner_addr = hex::encode(miner_key.verifying_key().as_bytes());

    // 4. 初始化账户余额
    bc.accounts.set_balance(&alice_addr, 100000);
    bc.accounts.set_balance(&bob_addr, 50000);

    // 5. 发起交易
    let tx = Transaction::new(&alice_key, &bob_addr, 10000);
    bc.add_transaction(tx).unwrap();

    // 6. 挖矿打包交易
    bc.mine_pending_transactions(&miner_addr);

    // 7. 验证链完整性
    assert!(bc.is_valid());

    // 8. 查询交易历史
    let history = bc.get_transaction_history(&alice_addr);
    for tx in history {
        println!("from: {} -> to: {}, amount: {}",
                 &tx.from[..16], &tx.to[..16], tx.amount);
    }
}
```

### 核心 API

#### BlockChain
- `new(difficulty)` - 创建新区块链
- `add_transaction(tx)` - 添加交易到交易池（验证签名和余额）
- `mine_pending_transactions(miner)` - 挖矿打包交易
- `is_valid()` - 验证链的完整性
- `get_transaction_history(address)` - 查询地址的交易记录
- `save2file(path)` - 保存到文件
- `load_from_file(path)` - 从文件加载

#### Transaction
- `new(signing_key, to, amount)` - 创建并签名交易
- `verify_signature()` - 验证交易签名
- `calculate_hash(...)` - 计算交易哈希

#### AccountManager
- `set_balance(address, balance)` - 设置账户余额
- `get_balance(address)` - 获取账户余额
- `transfer(from, to, amount, fee, miner)` - 执行转账
- `has_sufficient_balance(address, amount)` - 检查余额是否足够

---

## English

### Introduction
A complete blockchain implementation in Rust with Proof of Work (PoW), transaction system, digital signatures, and account management for learning blockchain principles.

### Features
- ✅ **Proof of Work (PoW)** - Configurable difficulty mining mechanism
- ✅ **Transaction System** - Complete transaction validation and mempool
- ✅ **Digital Signatures** - Ed25519 signature verification
- ✅ **Account Management** - Balance management and transfer functionality
- ✅ **Coinbase Transactions** - Miner reward mechanism
- ✅ **Chain Validation** - Integrity verification and tamper detection
- ✅ **Replay Protection** - Transaction hash deduplication
- ✅ **Transaction History** - Address transaction record queries
- ✅ **Data Persistence** - JSON serialization storage

### Quick Start

#### Run the project
```bash
cargo run
```

#### Build the project
```bash
cargo build --release
```

### Core Concepts

#### Transaction Flow
```
User initiates transaction → Sign with private key → Add to mempool → Validate (signature + balance + amount)
    ↓
Miner packages transactions → Mine (PoW) → New block → Update balances → Add to chain
```

#### Block Structure
- `index`: Block height
- `timestamp`: Creation timestamp
- `transactions`: List of transactions
- `prev_hash`: Hash of previous block
- `nonce`: Random number for proof of work
- `hash`: Hash of current block
- `miner`: Miner address

#### Transaction Structure
- `from`: Sender address (public key in hex)
- `to`: Recipient address
- `amount`: Transfer amount
- `fee`: Transaction fee
- `timestamp`: Timestamp (replay protection)
- `signature`: Ed25519 digital signature
- `hash`: Transaction hash

#### Proof of Work (PoW)
Continuously tries different nonce values to find a hash that meets the difficulty requirement (number of leading zeros).

#### Difficulty Configuration
Specify difficulty when creating the blockchain:
```rust
let blockchain = BlockChain::new(2);  // Difficulty 2 (requires 2 leading zeros)
```

#### Coinbase Transactions
- Miner reward mechanism for mining
- Special marker: `from = "coinbase"`
- Creates new coins from nothing, doesn't deduct from any account
- Skips signature verification
- Placed as the first transaction in a block

### Project Structure
```
mini-blockchain/
├── src/
│   ├── main.rs          # Main program and demo code
│   ├── block.rs         # Block structure and mining logic
│   ├── blockchain.rs    # Blockchain core logic
│   ├── transaction.rs   # Transaction and signature verification
│   └── account.rs       # Account and balance management
├── Cargo.toml           # Project configuration
├── README.md            # Documentation
└── coinbase.md          # Coinbase transaction explanation
```

### Tech Stack
- **Rust** - Programming language
- **sha2** - SHA-256 hashing algorithm
- **hex** - Hexadecimal encoding
- **ed25519-dalek** - Ed25519 digital signatures
- **serde** - Serialization/deserialization
- **rand** - Random number generation

### Usage Example

#### Basic Workflow
```rust
use mini_blockchain::blockchain::BlockChain;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() {
    // 1. Create blockchain
    let mut bc = BlockChain::new(2);

    // 2. Generate account key pairs
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);
    let miner_key = SigningKey::generate(&mut OsRng);

    // 3. Get addresses (public key in hex)
    let alice_addr = hex::encode(alice_key.verifying_key().as_bytes());
    let bob_addr = hex::encode(bob_key.verifying_key().as_bytes());
    let miner_addr = hex::encode(miner_key.verifying_key().as_bytes());

    // 4. Initialize account balances
    bc.accounts.set_balance(&alice_addr, 100000);
    bc.accounts.set_balance(&bob_addr, 50000);

    // 5. Initiate transaction
    let tx = Transaction::new(&alice_key, &bob_addr, 10000);
    bc.add_transaction(tx).unwrap();

    // 6. Mine and package transactions
    bc.mine_pending_transactions(&miner_addr);

    // 7. Verify chain integrity
    assert!(bc.is_valid());

    // 8. Query transaction history
    let history = bc.get_transaction_history(&alice_addr);
    for tx in history {
        println!("from: {} -> to: {}, amount: {}",
                 &tx.from[..16], &tx.to[..16], tx.amount);
    }
}
```

### Core API

#### BlockChain
- `new(difficulty)` - Create new blockchain
- `add_transaction(tx)` - Add transaction to mempool (validates signature and balance)
- `mine_pending_transactions(miner)` - Mine and package transactions
- `is_valid()` - Verify chain integrity
- `get_transaction_history(address)` - Query transaction records for an address
- `save2file(path)` - Save to file
- `load_from_file(path)` - Load from file

#### Transaction
- `new(signing_key, to, amount)` - Create and sign transaction
- `verify_signature()` - Verify transaction signature
- `calculate_hash(...)` - Calculate transaction hash

#### AccountManager
- `set_balance(address, balance)` - Set account balance
- `get_balance(address)` - Get account balance
- `transfer(from, to, amount, fee, miner)` - Execute transfer
- `has_sufficient_balance(address, amount)` - Check if balance is sufficient
