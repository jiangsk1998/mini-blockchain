use std::collections::HashMap;
use serde::{Deserialize, Serialize};

///漳湖与余额管理
#[derive(Debug, Clone)]
struct Account {
    address: String, //地址 公钥十六进制
    balance: u64,    //余额
}

///账户管理器（简化版 实际区块链用UXTO或状态树）
#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct AccountManager {
    // HashMap k 地址 v 余额  hashmap的key需要拥有所有权
    accounts: HashMap<String, u64>,
}
impl AccountManager {
    pub fn new() -> Self {
        AccountManager {
            accounts: HashMap::new(),
        }
    }

    ///初始化账户余额
    fn set_balance(&mut self, address: &str, balance: u64) {
        self.accounts.insert(address.to_string(), balance);
    }

    ///获取余额
    fn get_balance(&self, address: &str) -> u64 {
        //self.accounts.get(address).copied().unwrap_or(0)
        *self.accounts.get(address).unwrap_or(&0)
    }

    ///执行转账（不检查，只修改余额）
    fn transfer(&mut self, from: &str, to: &str, amount: u64) {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);
        self.accounts
            .insert(from.to_string(), from_balance - amount);
        self.accounts.insert(to.to_string(), to_balance - amount);
    }

    /// 检查余额是否足够
    fn has_sufficient_balance(&mut self, address: &str, amount: u64) -> bool {
        self.accounts.contains_key(address) || self.get_balance(address) >= amount
    }
}
