// Ownership
// 所有者是 rust 用来管理内存的。
// rust 不使用垃圾回收机制管理内存，也不需要显示申请释放内存。
// 采用 基于编译时的所有者检测体系，如果检测不通过，则编译失败。

// 所有者规则,需要牢记！！
// 1. Each value in Rust has an owner. (每个值都有一个所有者）
// 2. There can only be on owner at a time.（同一时刻，只归属一个所有者）
// 3. When the owner goes out of scope, the value will be dropped.   (当所有者离开作用域时，该值被丢弃）

// 引用规则
// 1.At any given time, you can have either one mutable reference or any number of immutable references. (同一时刻，不能既有可变，也有不可变)
// 2.References must always be valid. (引用必须始终有效，不能指向游离指针）

// Slice Type( 集合切片）

fn main() {
    // Basic ( smailer with other languages)
    {
        // s is not valied
        let s = "Hello";
        // do something with s
    } // this scope is over, and s is no longer valid.

    {
        // create a string on heap.
        let mut s = String::from("Hello");
        s.push_str(", world");
        println!("{s}");
        // 离开作用域时，rust 会调用 drop 函数释放内存, 这里类似 C++ RAII （Resource Acquisition Is Initialization )
    } // this scope is over ,and is is no longer valid.

    {
        // allocate a memmory on heap.
        // s1 (ptr, len, capacity) -> memory( h, e, l, l, o,)
        // ptr point at 'h' (index 0), len is how much memory in bytes that current used.
        // the capacity is the total amount of memory in bytes that the String has received from allocator.
        let s1 = String::from("Hello"); // heap

        // assign s1 to s2.
        // the string data is copied. (ptr, len, capacity) which on stack.
        // and data on heap will not be copy. s1 & s2 point at them same pointer on heap.
        let s2 = s1;

        // 此时 s2 s1 指向同一个内存，离开作用域时，会释放两次，（double free）会导致内存异常。
        // 因此：
        // 赋值之后，s1 会变为不可用(not valid),所以当离开作用域时，s1 不会释放内存。
        // println!("{s1} world"); // s1 value borrowed here after move

        // 这种赋值之后，s1 不可用, s2 可用的行为，在rust 中称为：移动（move）

        // 以上 基于，rust 永远不会自动 进行深拷贝！
        println!("{s2}, world");
    }

    {
        let mut s = String::from("Hello");
        // 'Hello' on heap will be freed by function 'drop'
        s = String::from("ahoy");
        println!("{s}, world"); // ahoy, world.
    }

    {
        let s1 = String::from("Hello");
        // 这里是深拷贝，因此 堆上有两份数据，s1，s2 指向不同的数据。
        let s2 = s1.clone();
        println!("s1: {s1} s2: {s2}"); // hello, hello.
    }

    // functions.
    {
        let s = String::from("Hello"); // s is valid
        take_ownership(s); // s will be moved into funciton. so here is is not valid.
        // println!("{s}"); // value borrowed here after move

        let x = 5;
        make_copy(5); // i32 is Copy, so, here x is still valid.
    }

    // return values
    {
        let s1 = gives_ownership(); // "Hello" ownership moved into s1.
        let s2 = String::from("world"); // s2 comes into scope.
        let s3 = takes_and_gives_back(s2); // s2 moved into function, and moved back to s3.
    } //  此处：s1 出作用域被 drop掉。s2 被移动到s3，不会drop。s3 出作用域，被drop。

    // References!!
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // s1 not moved into function. s1 is valid.
        println!("The length of '{s1}' is {len}.");
    }

    {
        // s 不可变，因此对应的 引用也不可变，无法修改。
        let s = String::from("hello");
        change(&s);
    }

    {
        // s 可变
        let mut s = String::from("hello");
        //需要传入可变引用
        change2(&mut s);
        println!("111 s is {s}")
    }

    {
        let mut s = String::from("Hello");
        let r1 = &mut s;
        // 可变借用只能一次，编译报错！
        // 主要是防止 多个可变导致数据竞争问题
        let r2 = &mut s; //error: second mutable borrow occurs here
        // println!("{r1}, {r2}");
    }

    {
        let mut s = String::from("Hello");
        {
            let r1 = &mut s;
        } // r1 goes out of scope, so r1 is invalid, and r2 reference is okay.
        let r2 = &mut s;
        println!("{r2}");
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        // r1 & r2 是不可变引用，因此不能添加 可变引用了。
        // let r3 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable
        // println!("r1 {r1} r2 {r2} r3 {r3}");
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("r1 {r1} r2 {r2}");

        // 这里与上面的测试不同在于：
        // 借用（引用）的作用域范围： 从申明开始，到最后一次使用！WTF???
        // 所以到这里的时候，r1 r2 已经无效了, 因此 r3 正常。
        let r3 = &mut s;
        println!("mutable r3 {r3}");
    }

    // dangling Refrence.
    {
        // let refrence_to_nothing = dangle();
    }

    // Slice Type 测试，详情见函数内部
    slice();
}

// // error: expected named lifetime parameter
// fn dangle() -> &String { // 返回一个 string 引用
//     let s = String::from("hello");// 创建一个string
//
//     &s // 返回一个引用
// } // s 离开作用域, 会被drop掉，因此 &s 是游离的
//
fn change(some_string: &String) {
    // some_string.push_str(", world"); // some_string` is a `&` reference, so it cannot be borrowed as mutable
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let string = String::from("Hello");
    return string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// ===============================================================================
fn slice() {
    {
        let s = String::from("Hello world");
        let index = first_word(&s);
        println!("slice first word index : {index}")
    }

    {
        let s = String::from("Hello World");
        // let hello = &s[0..5]; // index 0 - index 4
        let hello = &s[..5]; // same with above

        // let world = &s[6..11]; // index 6 - index 10
        let world = &s[6..]; // 6 to len
        println!("slice:  {hello} {world}");

        let whole = &s[..]; // 0 -> len
        println!("slice : whole {whole}");
    }

    {
        let s = String::from("Hello world");
        let word = first_word_str(&s);
        // word is immutable.
        println!("slice first word!: {word}");
    }
}

// 对string切片，返回第一个空能的索引, 如果没有空格，返回长度
fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes(); // 转换为字符数组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

// &str 专门用来表示 字符切片(string slice)!!! 绝了。。WTF。
// 是只读的
fn first_word_str(s: &String) -> &str {
    let bytes = s.as_bytes(); // 转换为字符数组
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
