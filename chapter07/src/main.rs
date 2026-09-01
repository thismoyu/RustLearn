// Packages, Crates, and Modules
//
// 前言:
// Rust 文件组织概要（法则）
// 总体来说，文件是一个树形
//
// 1.起点（crate root)
// 1) 对于可执行程序（binary）的根是 src/main.rs
// 2）对于库的根是 src/lib.rs
// 编译时，编译器先找根文件
//
// 2. 申明模块 (mod 关键字)
// 当在main.rs 中添加 mod garden;时编译器会按照以下顺序查找代码：
// 1) 内联代码：直接写在花括号里面的 mod garden {...}
// 2) 同名文件：找 src/garden.rs (推荐写法)
// 3) 同名文件夹下的mod.rs: 找 src/garden/mod.rs
//
// 3. 申明子模块 （submodule)
// 如果在 src/garden.rs 中添加 mod vegetables; 时，编译器会在 garden 文件夹中查找
// 1) 内联代码：直接写在花括花里面的 mod vegetables {...}
// 2) 查找 src/garden/vegetables.rs 文件
// 3) 查找 src/garden/vegetables/mod.rs 文件
//
// 4. 通过路径指定代码
// 如果已经添加了一个模块，那么就可以在crate的任何地方引入代码，只要隐私允许
// 例如：crate::garden::vegetables::Asparagus
//
// 5. 私有和公开
// 模块中的代码默认是私有的，且默认对父模块不可见
// 如果要设置为公开的，
// 1) 模块需要添加 pub mod
// 2) 模块中的结构体/函数也要添加pub
//
// 6. use 关键字
// 使用use 减少代码,例如
// use crate::garden::vegetables::Asparagus;
// 那么作用域内，可以直接使用 Asparagus
//
// 例子
// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs

// Packages: A Cargo feature that lets you build, test, and share crates
// Crateds: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Path: A way of naming an item, such as a struct, function, or module

// Crates : 是 Rust 编译的最小代码合集, crates 中可以包含 modules.
// 它有两种形式：1. 二进制(binary)文件 2.库（library）文件
// 二进制 可以用于编译成可执行文件，在命令行执行或者作为一个服务, 必须包含一个 main 函数作为入口
// 库 没有 main 入口函数，不能编译为可执行文件，是分享给其他项目的功能集

// Package: 是 crates 的集合
// 一个 Package 包含一个 Cargo.toml 用于描述 crates 怎么build
// (Cargo 本身就是一个 package，包含了一堆编译命令等, 和一堆库）
// 此学习项目就是一个 Package
// 一个 package 可以包含任意个 binary crate ,但是 最多只有一个 library crate.
// 一个 package 最少包含一个 crate. 无论是 binary 还是 library

// Modules: 这个应该就是 java 中的package， python 中的module, c++ 中的namespace

// use 关键字
use crate::garden::vegetables::Asparagus;

pub mod garden; // 告诉编译器，加载 garden.rs

//
// use std::cmp::Ordering;
// use std::io;
// 等价于
use std::{cmp::Ordering, io };


// use std::io;
// use std::io::Write;
// 等价于, self 表示 io 自身
// use std::io::{self, Write};


// 通配符，全部导入
use std::cmp::*;

fn main() {
    let plant = Asparagus {};

    // 全路径
    chapter07::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    chapter07::eat_at_restaurant();

    // use-as 关键字
    use chapter07::front_of_house::hosting as front_house;
    front_house::add_to_waitlist();

    // 这里减少了 front_of_house 层级，依赖于 lib.rs 中的 re-export 技术
    chapter07::hosting::add_to_waitlist();
}
