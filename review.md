# Rust 复习要点

## 1. serde 自定义序列化 `#[serde(with = "module")]`

### 什么时候用？
当结构体字段的类型**没有实现** `Serialize`/`Deserialize` trait 时（比如第三方库的类型），
需要自己写序列化/反序列化逻辑。

### 怎么用？
```rust
// 1. 定义一个模块，包含 serialize 和 deserialize 两个函数
mod signature_serde {
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(value: &目标类型, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer { ... }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<目标类型, D::Error>
    where D: Deserializer<'de> { ... }
}

// 2. 在字段上标注使用该模块
#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "signature_serde")]  // ← 指向上面的模块
    signature: Signature,
}
```

### 数据流转
```
序列化：Rust类型 → 转为可序列化的中间格式(字节/字符串) → JSON
反序列化：JSON → 解析为中间格式 → 还原为Rust类型
```

### 其他常用 serde 属性
| 属性                         | 作用             |
|------------------------------|------------------|
| `#[serde(with = "mod")]`    | 自定义序列化     |
| `#[serde(rename = "name")]` | 重命名字段       |
| `#[serde(default)]`         | 缺失时用默认值   |
| `#[serde(skip)]`            | 跳过该字段       |
| `#[serde(flatten)]`         | 展平嵌套结构     |

---

## 2. Rust 模块系统

### 文件即模块
```
src/
├── main.rs        // mod block; mod blockchain; → 声明模块
├── block.rs       // 对应 block 模块
└── blockchain.rs  // 对应 blockchain 模块
```

### 关键语法
- `mod xxx;` — 声明模块（告诉编译器去找 xxx.rs）
- `pub` — 公开，其他模块可访问
- `use crate::block::Block;` — 从项目根路径导入
- 不加 `pub` 的函数/字段是私有的，只能在模块内部使用

---

## 3. 迭代器 fold 方法
```rust
// fold(初始值, |累加器, 当前元素| 返回新的累加器值)
let total = vec.iter().fold(0, |acc, x| acc + x.data.len());

// 等价的 for 循环写法：
let mut total = 0;
for x in vec.iter() {
    total = total + x.data.len();
}
```

---

## 4. ed25519-dalek 版本差异
- **1.x**: `Signature::from_bytes(&[u8])` → 返回 `Result<Signature, Error>`
- **2.x/3.x**: `Signature::from_bytes(&[u8; 64])` → 直接返回 `Signature`（固定长度不会失败）

注意：网上很多教程基于旧版，遇到 `.map_err()` 报错时检查版本。

2---

## 5. HashMap::get 的引用与解引用

### 代码示例
```rust
*self.accounts.get(address).unwrap_or(&0)
```

### 逐步拆解

| 步骤 | 代码 | 类型 | 说明 |
|------|------|------|------|
| 1 | `self.accounts.get(address)` | `Option<&u64>` | `get()` 返回值的**引用** |
| 2 | `.unwrap_or(&0)` | `&u64` | 默认值必须匹配 `&u64` 类型，所以传 `&0` |
| 3 | `*...` | `u64` | 解引用，拿到实际的值 |

### 为什么是 `&0`？
`HashMap::get()` 返回 `Option<&V>`，`unwrap_or` 的参数类型必须与之一致，即 `&V`，所以传 `&0` 而不是 `0`。

### 为什么要 `*`？
`unwrap_or(&0)` 的结果是 `&u64`（引用），用 `*` 解引用得到 `u64` 值本身。

### 更简洁的等价写法
```rust
self.accounts.get(address).copied().unwrap_or(0)
```
`copied()` 将 `Option<&u64>` 转为 `Option<u64>`，之后 `unwrap_or` 直接传 `0` 即可。

---

## 6. Rust panic 错误信息解读

### 错误示例
```
thread 'main' panicked at src\account.rs:41:46:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\mini-blockchian.exe` (exit code: 101)
```

### 逐段解析

**`thread 'main' panicked at src\account.rs:41:46`**
- `thread 'main'` — 崩溃发生在主线程
- `panicked` — Rust 遇到无法恢复的错误，程序立即终止
- `src\account.rs:41:46` — 精确位置：文件名、行号、列号（列号指向出错的运算符）

**`attempt to subtract with overflow`**

核心错误原因。Rust 的 `u64` 是无符号整数，最小值为 0，不能表示负数。

```rust
// u64 类型，to_balance = 0，amount = 50
to_balance - amount  // 0 - 50 → 结果为负数 → u64 无法表示 → panic！
```

- **Debug 模式**（`cargo run` / `cargo build`）：溢出时直接 panic，保护开发者
- **Release 模式**（`cargo run --release`）：溢出时静默回绕（0 - 50 变成一个极大的数），更危险

**`note: run with RUST_BACKTRACE=1 ...`**

设置环境变量可查看完整调用栈（从 `main` 到出错位置的函数调用链）：

```bash
# Windows
set RUST_BACKTRACE=1 && cargo run

# 输出示例：
# 0: rust_begin_unwind
# 1: core::panicking::panic_fmt
# 2: mini_blockchian::account::AccountManager::transfer  ← 出错位置
# 3: mini_blockchian::blockchain::BlockChain::mine_pending_transactions
# 4: mini_blockchian::main
```

**`exit code: 101`**

Rust `panic!` 的固定退出码，表示程序因 panic 异常退出（正常退出是 `exit code: 0`）。

### 快速定位 panic 的步骤
1. 看文件名和行号 → 直接跳到出错代码
2. 看错误描述（`attempt to subtract with overflow` / `index out of bounds` 等）
3. 开启 `RUST_BACKTRACE=1` 看调用链，找到是谁触发了这行代码

---

## 7. 整数截断导致小额手续费为零

### 问题现象

手续费率为 1%，转账金额为 50 时，手续费计算结果为 0：

```rust
// 浮点写法
let fee = (50_u64 as f64 * 0.01) as u64;  // 0.5 → 截断为 0

// 整数写法
let fee = 50_u64 / 100;  // 整数除法，0.5 → 截断为 0
```

### 根本原因

`u64` 没有小数，最小单位是 1。当转账金额 < 100 时，1% 的结果不足 1，截断后变成 0。这是整数类型在做百分比计算时的通用陷阱。

### 三种解决方案

**方案 A：设最低手续费（最简单）**
```rust
let fee = (amount / 100).max(1); // 至少收 1
```
缺点：小额交易实际手续费比例远高于 1%。

**方案 B：用更小的费率单位（基点）**
```rust
const FEE_RATE: u64 = 10;           // 万分之十
let fee = amount * FEE_RATE / 10000; // 转 1000 → 手续费 1
```
将费率单位从"百分之一"细化为"万分之一"，阈值从 100 降到 10000，但根本问题未变。

**方案 C：账本使用最小单位（类比比特币 satoshi）**

账本中 1 代表最小单位（如 0.01 元），显示时除以 100：
```
账本余额 100000 = 实际 1000 元
转账 5000（= 50 元），手续费 = 5000 / 100 = 50（= 0.5 元）
```
这是真实区块链的做法，精度最高，但需要整体重新设计金额体系。

### 结论

| 方案 | 实现成本 | 精度 | 适用场景 |
|------|----------|------|----------|
| 最低手续费 `.max(1)` | 最低 | 差 | 学习项目 |
| 基点费率 | 低 | 中 | 简单项目 |
| 最小单位账本 | 高 | 最高 | 生产系统 |
