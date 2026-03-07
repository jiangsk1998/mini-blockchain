use sha2::{Sha256, Digest};

/// 计算 SHA-256 哈希
/// 参数：任意字节切片
/// 返回：64 字符的十六进制哈希字符串
pub fn sha256_hash(data: &[u8]) -> String {
    // 链式调用拆解：
    //   Sha256::digest(data)         → 对字节数据做 SHA-256 哈希，返回 GenericArray<u8, 32>
    //   hex::encode(...)             → 将 32 字节哈希值编码为 64 位十六进制字符串
    hex::encode(Sha256::digest(data))
}

/// 计算字符串的 SHA-256 哈希
/// 参数：字符串切片
/// 返回：64 字符的十六进制哈希字符串
pub fn sha256_str(input: &str) -> String {
    // input.as_bytes() → 将 &str 转为 &[u8] 字节切片
    sha256_hash(input.as_bytes())
}