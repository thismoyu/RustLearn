# 结构

项目采用 workspace 结构，见 Cargo.tml:

```toml
[workspace]

resolver = "3"

members = [
    "chapter*",
]

```

将所有 chapter* 目录当作 package



## 创建新章节

```bash
cargo new chapter01
```



## 运行指定章节

### 在项目根目录下运行

```bash
cargo run -p chapter01
```

### 进入子目录运行

```bash
cd chapter01
cargo run
```

### 一键运行所有

```
# 快速语法检查所有章节代码（最常用，速度极快）
cargo check

# 编译所有章节
cargo build

# 运行所有章节里的单元测试
cargo test
```

