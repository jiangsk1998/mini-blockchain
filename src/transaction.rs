use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

mod signature_serde {
    use ed25519_dalek::Signature;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(sig: &Signature, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 将 Signature 转为字节数组，再转为十六进制字符串
        serializer.serialize_str(&hex::encode(sig.to_bytes()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 从十六进制字符串解码为字节数组，再转为 Signature
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        let arr: [u8; 64] = bytes.try_into()
            .map_err(|_| serde::de::Error::custom("签名长度错误"))?;
        Ok(Signature::from_bytes(&arr))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    from: String,   //发送方地址（公钥）  公钥的十六位置字符串，需要解码
    to: String,     //接收方地址
    amount: u64,    //转账金额
    timestamp: u64, // 时间戳（防重放）

    #[serde(with = "signature_serde")]
    signature: Signature, // Ed25519 签名（证明身份）
}

// 交易的创建与签名
//1.构造交易数据
//2.计算交易hash(SHA-256)
//3.私钥签名哈希
//4.附上签名，得到完整交易

impl Transaction {
    ///创建新交易，自动签名
    /// 参数：发送方私钥，接收方地址，金额
    fn new(signing_key: &SigningKey,to:&str,amount:u64) -> Self {
        //获取发送方地址（公钥）
        let from = hex::encode(signing_key.verifying_key().as_bytes());
        let to = to.to_string();
        let timestamp = Self::current_timestamp();

        //计算交易hash,不包含签名
        let tx_hash = Self::calculate_hash(&from, &to, amount, timestamp);

        //用私钥签名hash
        let signature = signing_key.sign(tx_hash.as_bytes());

        Transaction{
            from,
            to,
            amount,
            timestamp,
            signature
        }

    }

    //计算当前时间戳
    fn current_timestamp() -> u64 {
        use std::time::SystemTime;
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }

    //计算交易Hash 用于签名
    fn calculate_hash(from: &str, to: &str, amount: u64, timestamp: u64) -> String {
        let input = format!("{}{}{}{}", from, to, amount,timestamp);
        hex::encode(Sha256::digest(input.as_bytes()))
    }
}

///交易验证
impl Transaction {
    fn verify_signature(&self) -> bool {
        //拿到公钥
        
        //解码拿到公钥字符串数组
        let public_key_arr = hex::decode(&self.from).unwrap().try_into().unwrap();
        //从字符串数组获取公钥 调用Ed25519的VerifyingKey::from_bytes
        let verifying_key = VerifyingKey::from_bytes(&public_key_arr).unwrap();

        //重新计算交易hash
        let tx_hash = Transaction::calculate_hash(&self.from, &self.to, self.amount, self.timestamp);
        //验证签名
        verifying_key.verify(tx_hash.as_bytes(),&self.signature).is_ok()
    }
}


