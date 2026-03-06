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
