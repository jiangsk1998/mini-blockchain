// src/blockchain/mod.rs
// 声明子模块（告诉编译器：blockchain 目录下有这些文件）
pub mod block;          // → 对应 src/blockchain/block.rs
pub mod transaction;    // → 对应 src/blockchain/transaction.rs
pub mod blockchain;          // → 对应 src/blockchain/chain.rs
pub mod account;        // → 对应 src/blockchain/account.rs

// 可以在这里 re-export（重新导出），让外部使用更方便
// pub use 的作用：让外部可以直接 use blockchain::Block，而不需要 use blockchain::block::Block
// Java 类比：相当于在 package-info.java 中定义公共导出
pub use block::Block;
pub use transaction::Transaction;
pub use blockchain::BlockChain;
pub use account::AccountManager;