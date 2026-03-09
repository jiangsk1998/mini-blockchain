use sha2::{Digest, Sha256};
use std::fmt::format;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct MerkleProofStep {
    pub hash: String,  // 兄弟节点的哈希
    pub is_left: bool, // true: 兄弟在左边，false: 兄弟在右边
}

impl MerkleProofStep {
    /// 构建默克尔路径
    /// 参数：
    ///   - tx_index: 要验证的交易在区块中的索引
    ///   - all_tx_hashes: 区块中所有交易的哈希列表
    /// 返回：默克尔路径
    fn build_merkle_proof(tx_index: usize, all_tx_hashes: &[String]) -> Vec<MerkleProofStep> {
        let mut proof = Vec::new(); //构建返回值
        let mut current_level = all_tx_hashes.to_vec(); //如果是鸡数需要复制一个节点塞进去。第一层的tx_Hash集合
        let mut index = tx_index;

        //长度大于一说明存在数
        while current_level.len() > 1 {
            // 如果是奇数个，复制最后一个

            if current_level.len() % 2 != 0 {
                current_level.push(current_level[current_level.len() - 1].clone());
            }

            // 判断当前节点是左孩子还是右孩子
            let is_left = index % 2 == 0; //能被2整除 奇数 左边

            // 兄弟节点的索引
            let bro_index = if is_left { index - 1 } else { index + 1 };

            // 记录兄弟节点的哈希和位置
            proof.push(MerkleProofStep {
                hash: current_level[bro_index].clone(),
                is_left: !is_left,
            }); //兄弟节点的位置和我相反

            // 构建下一层
            let mut next_level = Vec::new();
            for pair in current_level.chunks(2) {
                let combined = format!("{}{}", pair[0], pair[1]);
                next_level.push(hex::encode(Sha256::digest(combined.as_bytes())));
            }

            current_level = next_level;
            index = index / 2; // 向上一层
        }

        proof
    }

    /// 验证默克尔路径
    /// 参数：
    ///   - tx_hash: 要验证的交易哈希
    ///   - proof: 默克尔路径
    ///   - expected_root: 期望的默克尔根（来自区块头）
    /// 返回：true 表示验证通过
    fn verify_merkle_proof(
        tx_hash: &str,
        proof: Vec<MerkleProofStep>,
        expected_root: &str,
    ) -> bool {
        let mut current_hash = tx_hash.to_string();
        for step in proof {
            let combined: String = if step.is_left {
                format!("{}{}", step.hash, current_hash)
            } else {
                format!("{}{}", current_hash, step.hash)
            };
            current_hash = hex::encode(Sha256::digest(combined.as_bytes()))
        }
        current_hash == expected_root
    }
}
