# RustLearn

Rust 学习笔记与实践代码，按章节组织，对应《The Rust Programming Language》等常见教程的章节顺序。

## 章节说明

| 章节 | 主题 | 简要说明 |
|------|------|----------|
| chapter01 | Hello, world! | Rust 程序入口、`println!` 宏与最基础的程序结构 |
| chapter02 | 猜数字游戏 | 变量与可变性、`rand` 依赖、`match` 比较、`loop` 循环与错误输入处理 |
| chapter03 | 常见概念 | 变量、常量、遮蔽（shadowing）、函数、条件分支与各类循环（`loop` / `while` / `for`） |
| chapter04 | 所有权 | 所有权三规则、移动与克隆、引用与借用、可变引用限制、字符串切片（`&str`） |
| chapter05 | 结构体 | 结构体定义与实例化、元组结构体、方法（`impl`）、关联函数与 `Debug` 格式化 |
| chapter06 | 枚举与模式匹配 | 枚举定义与变体数据、`Option` 类型、`match` / `if let` / `let else` 模式匹配 |
| chapter07 | 包、Crate 与模块 | Package/Crate 概念、模块树与文件组织（`mod`）、路径（绝对/相对）、`pub` 可见性、`use` 与 re-export、库 crate（`lib.rs`）与二进制 crate（`main.rs`） |
| chapter08 | （待补充） | 常见集合：`Vec`、`String`、`HashMap` 等（章节内容待编写） |

## 结构

项目采用 workspace 结构，见 Cargo.toml:

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

