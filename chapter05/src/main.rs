// Using Structs to Structure Related Data
//
// 1. 定义 结构体

// 2. 方法（Method), 定义在结构体/枚举等内不，方法第一个参数是self
// Unlike functions, methods are defined within the context of a struct
// (or an enum or a trait object, which we cover in Chapter 6 and Chapter 18, respectively),
// and their first parameter is always self, which represents the instance of the struct the method is being called on.

// 以下是 Rust没有 -> 操作符的原因，简单来说，就是Rust 会自动引用和解引用
// Where’s the -> Operator?
// In C and C++, two different operators are used for calling methods: You use .
// if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first.
// In other words, if object is a pointer, object->something() is similar to (*object).something().
//
// Rust doesn’t have an equivalent to the -> operator;
// instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.
//
// Here’s how it works: When you call a method with object.something(),
// Rust automatically adds in &, &mut, or * so that object matches the signature of the method. In other words, the following are the same:
//
// p1.distance(&p2);
// (&p1).distance(&p2);
// The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self.
// Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self).
// The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
//

// normal struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like struct.
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Rust 不允许 仅将部分字段改为 mutable
    let mut user1 = User {
        active: true,
        username: String::from("somename123"),
        email: String::from("someemail@123.com"),
        sign_in_count: 1,
    };

    user1.active = false;
    user1.email = String::from("someema@example");
    println!("{0}", user1.email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };
    println!("{0}", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        // 这里只能使用user2， 因为user1中的已经被move到user2了
        // 表示，除了email 之外，剩下的值使用user2的值
        ..user2
    };

    // tuple struct.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like struct.
    let subject = AlwaysEqual;

    // example
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of rectangle is {} square pixels.",
        area(width1, height1)
    );
    // tuple
    let rect1 = (30, 50);
    println!("The area of rectangle is {} square pixels.", area_2(rect1));

    // struct
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels.", area_3(&rect2));
    // println!("{}", rect2);// Rectangle` cannot be formatted with the default formatter

    // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
    // error: `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
    // note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
    println!("{rect2:?}");
    // 保持格式，添加#
    println!("{rect2:#?}");

    // dbg! macro
    // dbg! 宏会持有 所有权，并打印代码行数，然后返回值 和所有权
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale), // 这里会返回表达式的值的所有权
        height: 50,
    };

    // 这里报错原因是，rect3 直接传递给了 dbg, 且没有hold 返回值，因此rect3 失去了所有权
    // dbg!(rect3);
    // println!("{:#?}",rect3);//error: value borrowed here after move

    dbg!(&rect3);
    println!("{:#?}", rect3); // this is okay

    // Method
    println!("The area of rectangle is {} square pixels.", rect3.area());
    if rect3.width() {
        println!("The rectangle has a nonzero width; it is {}", rect3.width);
    }

    let rect11 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect22 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect33 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect11.can_hold(&rect22));
    println!("Can rect1 hold rect3? {}", rect11.can_hold(&rect33));

    // 使用关联函数创建一个方形（静态函数）
    let sq = Rectangle::square(100);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_2(dimeensions: (u32, u32)) -> u32 {
    dimeensions.0 * dimeensions.1
}
fn area_3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block, 此代码块中的代码与 Rectangle 绑定
impl Rectangle {
    // Method
    // &self 等价于 self: &Self, 在此代码块中 Self 是 Rectangle 的别名
    // 因此等价于 self: &Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Associated Functions.
    // 定义在 impl 块中的函数是 关联函数(不持有 self 参数）
    // 该函数不依赖 一个实例
    // 静态函数？
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// mutiple impl block
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
