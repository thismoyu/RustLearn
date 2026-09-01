// Enums and Pattern Matching

// 此章节有 rust关于null的说明
// 即，Rust中默认采用 Option 枚举
// enum Option<T> {
//     None,
//     Some(T),
// }
// Rust doesn’t have the null feature that many other languages have
// Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.
// As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>

// Option 枚举默认包含，因此使用的时候不需要添加 Option::前缀

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
//-----------------------------------------
enum IpAddr2 {
    V4(String),
    V6(String),
}

// Each variant can have different types and amounts of associated data.
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// -------------------------------------------
// same with std.
struct Ipv4Addr {}
struct Ipv6Addr {}

// include another struct / enum.
enum IpAddr4 {
    V4(Ipv4Addr),
    v6(Ipv6Addr),
}

// -----------------------------------------------
enum Message {
    // no data
    Quit,
    // like struct
    Move { x: i32, y: i32 },
    // single parameter.
    Write(String),
    // three i32
    ChangeColor(i32, i32, i32),
}

// ? 真特么复杂，枚举类跟 结构体一样可以定义 方法(Method)
impl Message {
    fn call(&self) {
        // method body
    }
}

// -----------------------------------------------
// Match
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1960,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// 原函数 describe_state_quarter
// 在 if let 语句中添加了条件判断，在复杂的业务下，会导致分支 可读性下降
// 改为，使用 if let 语句，尽早的返回
fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// 改为 let else 语句增加可读性
fn describe_state_quarter3(coin: Coin) -> Option<String> {
    // 解构coin，提取state，如果失败，直接返回
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {state:#?}");
            25
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // -------------------------------------------
    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V4(String::from("::1"));

    // -----------------------------------------
    let home3 = IpAddr3::V4(127, 0, 0, 1);

    // -----------------------------------------
    let m = Message::Write(String::from("hello"));
    m.call();

    // -----------------------------------------
    // Options
    let some_number = Some(5); // Option<i32>
    let some_char = Some('e'); // Option<char>

    // 空
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // error: cannot add `Option<i8>` to `i8`
    // let sum = x + y;// add no implementation for `i8 + Option<i8>
    let _sum = x + y.unwrap_or_default();

    // --------------------------------------
    // Match
    let v1 = value_in_cents(Coin::Penny);
    let v2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("penny {v1} quarter in Alaska {v2}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // --------------------------------------
    // if let 简写: 适用于处理 只关心一种匹配情况的场景
    let config_max = Some(3u8 /* 3 in u8 */);
    match config_max {
        Some(max) => println!(""),
        _ => (),
    }
    // 逻辑为：检查 config_max 是否匹配 Some 类型
    // 如果是 Some， 则将 config_max中的值解包出来，绑定到 max上
    // 然后执行代码块, 如果不是Some类型，则跳过
    // NOTE：
    // 这里比较绕，应该理解为
    // 首先 let Some(max) = config_max 用于从 config_max中取出 值，
    // 但是 为了匹配 非 Some 情况，前面价格 if
    // 表示：如果 从config_max 中成功提取了 Some(max) 则处理什么，
    if let Some(max) = config_max {
        println!("the max is {max}");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    // 简写为, 注意这里上面代码要注释，否则 coin 中的 state被转移到上面的 match中的state
    if let Coin::Quarter(us_state) = coin {
        println!("State quarter from {us_state:?}");
    } else {
        count += 1;
    }

    // --------------------------------------
}

fn route(_ip_kind: IpAddrKind) {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // must cover all posibilities.
    match x {
        Some(value) => Some(value + 1),
        // Cas1
        // None => None,

        // Case2
        // because the last pattern will match all values not specifically listed
        // 将其他情况都绑定值 other， other是一个值
        // other => None,

        // Case3
        // except the Some. like 'default'
        // _ is a special pattern that matches any value and does not bind to that value
        // 这个跟上面的区别是：这个指不绑定值
        _ => None,
    }
}
