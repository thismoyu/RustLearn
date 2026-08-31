use core::num;
use std::hint::black_box;

// Variables and Mutability
fn main() {
    let x = 5;
    println!("x is {x}");

    // x = 6;// cannot assign twice to immutable variable
    // println!("x is {x}");

    let mut y = 6;
    y = 10;
    println!("y is {y}"); // good

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("seconds is {THREE_HOURS_IN_SECONDS}");

    // shadow
    let a = 10;
    println!("a is {a}");
    {
        let a = 100; // this is a overshadow , original a not affect.
        println!("overshadow a is {a}");
    }
    println!("final a is {a}");

    let spaces = "  ";
    let spaces = spaces.len(); // shadow
    println!("space is {spaces}");

    // let mut spaces2 = "  ";
    // spaces2 = spaces2.len();// expected `&str`, found `usize`

    // let pa = "42".parse().expect("Not a number"); // type must be known at this point

    let type1 = b'A'; // u8 -> 65
    println!("type1 is {type1}");

    // overflow
    let mut oa: u8 = 255;
    // 按照官方说明，255 + 1 会回绕（wrapped），debug模式会直接报错，
    // 但是 release 模式下不会报错，
    // 在实际验证中发现：
    // 上面的写法，debug模式会报错，但是release模式下也会报错，原因是编译器优化，会自动识别到
    // 因此 使用 black_box 防止编译器在编译期优化和提前计算, 注意：此时 debug模式下也不会报错了
    // let mut oa : u8 = black_box(255);
    // let mut ob = oa + 1;
    // println!("ob is {ob}"); // 0

    let ob = oa.wrapping_add(1);
    println!("wrapping_add ob is {ob}");

    let ob = oa.checked_add(1);
    // if overflowed , print None.
    println!("checked_add ob is {ob:?}");

    let (ob, overflow) = oa.overflowing_add(1);
    println!("overflowing_add ob is {ob}, overflowed is {overflow}");

    let ob = oa.saturating_add(1);
    println!("saturating_add ob is {ob}");

    println!("----------------------------------------------");

    let y: u32 = {
        let x = 3;
        // 注意：这里不能加 分号。 如果加分号，表示是一个语句（statement）会编译报错
        // 语句没有返回值, 所以不能添加分号
        x + 1
    };
    println!("y is {y}");

    anothor_functions(5, 'h');
    let x = five();
    println!("five is {x}");
    let x = five_2();
    println!("five_2 is {x}");

    condition();
    loop_test();
    loop_lable();
    loop_other();
}

// functions
fn anothor_functions(x: i32, unit_lable: char) {
    println!("value is {x} char is {unit_lable}");
}

fn five() -> i32 {
    return 5;
}

fn five_2() -> i32 {
    // 此写法不能添加分号，会变为语句
    5
}

// condition
fn condition() {
    let number = 3;
    if number < 7 {
        println!("less than 7 ");
    } else {
        println!("bigger than 7 ");
    }
}

// loop
fn loop_test() {
    let mut count = 2;
    let result = loop {
        println!("loop");
        count += 1;
        if count == 10 {
            // 使用 count * 2 break掉, 第一次见。。。
            break count * 2;
        }
    };
    println!("result is {result}");
}

// loop with lable (must begin with signle quote)
fn loop_lable() {
    let mut count = 0;
    'counting_lable: loop {
        println!("count is {count}");

        let mut remaining = 10;
        loop {
            println!("remaining is {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_lable;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count is {count}");
}

fn loop_other() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element is {element}");
    }

    let mut b = 0;
    while b < 5 {
        println!("b is {b}");
        b += 1;
    }

    for number in (1..4) {
        println!("number is {number}");
    }
    // include 4.
    for number in (1..=4).rev() {
        println!("number is {number}");
    }

}
