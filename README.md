# Mini Blockchain | 迷你区块链

[中文](#中文) | [English](#english)

---

## 中文

### 简介
一个用 Rust 实现的简易区块链项目，包含工作量证明（PoW）机制，用于学习区块链核心原理。

### 特性
- ✅ 工作量证明（Proof of Work）挖矿机制
- ✅ 可配置的挖矿难度
- ✅ 区块链完整性验证
- ✅ 防篡改检测
- ✅ SHA-256 哈希算法

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

#### 区块结构
- `index`: 区块高度
- `timestamp`: 创建时间戳
- `data`: 交易数据
- `prev_hash`: 前一个区块的哈希
- `nonce`: 工作量证明的随机数
- `hash`: 当前区块的哈希

#### 工作量证明
通过不断尝试不同的 nonce 值，找到满足难度要求的哈希（前导零数量）。

#### 难度配置
创建区块链时可指定难度：
```rust
let blockchain = BlockChain::new(2);  // 难度为 2（需要 2 个前导零）
```

### 项目结构
```
mini-blockchian/
├── src/
│   └── main.rs          # 主程序
├── Cargo.toml           # 项目配置
└── README.md            # 说明文档
```

### 技术栈
- Rust
- sha2 (SHA-256 哈希)
- hex (十六进制编码)

---

## English

### Introduction
A simple blockchain implementation in Rust with Proof of Work (PoW) mechanism for learning blockchain fundamentals.

### Features
- ✅ Proof of Work mining mechanism
- ✅ Configurable mining difficulty
- ✅ Blockchain integrity validation
- ✅ Tamper detection
- ✅ SHA-256 hashing algorithm

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

#### Block Structure
- `index`: Block height
- `timestamp`: Creation timestamp
- `data`: Transaction data
- `prev_hash`: Hash of previous block
- `nonce`: Random number for proof of work
- `hash`: Hash of current block

#### Proof of Work
Continuously tries different nonce values to find a hash that meets the difficulty requirement (number of leading zeros).

#### Difficulty Configuration
Specify difficulty when creating the blockchain:
```rust
let blockchain = BlockChain::new(2);  // Difficulty 2 (requires 2 leading zeros)
```

### Project Structure
```
mini-blockchian/
├── src/
│   └── main.rs          # Main program
├── Cargo.toml           # Project configuration
└── README.md            # Documentation
```

### Tech Stack
- Rust
- sha2 (SHA-256 hashing)
- hex (hexadecimal encoding)
