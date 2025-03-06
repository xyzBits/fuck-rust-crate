use std::mem;

#[derive(Debug, Copy, Clone)]
struct StackStruct {
    a: i32,
    b: f64,
    c: bool,
}
#[test]
fn test_01() {
    let s1 = StackStruct {
        a: 1,
        b: 1.0,
        c: true,
    };
    // 此处发生复制，copy，因为结构体实现了 copy trait
    let s2 = s1;

    // 打印结构体地址，栈地址通常较高
    println!("address of s1: {:p}", &s1);
    println!("address of s2: {:p}", &s2);

    let addr_s1 = &s1 as *const StackStruct as usize;
    let addr_s2 = &s2 as *const StackStruct as usize;
    let diff = addr_s1.abs_diff(addr_s2);
    println!("address difference: {} bytes ", diff);
}

#[test]
fn test_02() {
    let x = 42;
    let y = 3.14;

    let addr_x = &x as *const i32 as usize;
    let addr_y = &y as *const f64 as usize;

    println!("address of x: {:p}", &x);
    println!("address of y: {:p}", &y);

    println!("integer address of x: {}", addr_x);
    println!("integer address of x: {}", addr_y);

    let diff = addr_x.abs_diff(addr_y);
    println!("address difference: {} bytes ", diff);
}

#[test]
fn test_03() {
    let stack_var = 42;
    let stack_var2 = 3.14;

    let heap_var = Box::new(42);
    let heap_var2 = Box::new(3.14);

    println!("address of stack_var: {:p}", &stack_var);
    println!("address of stack_var2: {:p}", &stack_var2);

    println!("address of heap_var(pointer): {:p}", &heap_var);
    println!("address of heap_var2(pointer): {:p}", &heap_var2);

    println!("address of heap_var(heap): {:p}", heap_var);
    println!("address of heap_var2(heap): {:p}", heap_var2);
}

#[test]
fn test_04() {
    let s1 = Box::new(StackStruct {
        a: 42,
        b: 3.14,
        c: true,
    });

    // s1 所有权转移，失效
    let s2 = s1;

    println!("stack address of s2: {:p}", &s2); // 栈地址，通常较高 0x63ccdff138
    println!("heap address of s1: {:p}", s2); // 堆地址，通常较低 0x23ca13bb3b0
}

/// num 是原始类型，存储在栈上
/// text string 类型由三部分组成
/// 指向堆内存的指针 ptr
/// 字符串长度 len
/// 堆内存容量 cap
/// 实际字符串数据，"hello" 存储在堆上
/// 结构体实例本身，num text元数据，存储在栈上
/// text的字符串数据，存储在堆上
struct MyStruct {
    num: i32,
    text: String,
}

#[test]
fn test_05() {
    let my_struct = MyStruct {
        num: 42,
        text: "Hello World!".to_string(),
    };

    println!("address of my_struct: {:p}", &my_struct);

    println!("address of text's metadata: {:p}", &my_struct.text);

    println!("address of text's data: {:p}", my_struct.text.as_ptr());
}

#[test]
fn test_06() {
    let mut s = MyStruct {
        num: 42,
        text: "Hello World!".to_string(),
    };

    let stolen_text = mem::replace(&mut s.text, String::new());

    println!("{}", stolen_text);
    println!("{}", s.num);
    println!("{}", s.text);
}

/// 所有权系统的灵活性，rust 允许精细控制每个字段的所有权，而非强制整个结构体失效
/// 内存安全，转移 String 后，原结构体的 text 字段置为无效值，避免悬垂引用
struct SafeStruct {
    num: i32,
    text: Option<String>,
}

#[test]
fn test_07() {
    let mut s = SafeStruct {
        num: 43,
        text: Some("Hello World!".to_string()),
    };

    let stolen_text = s.text.take().unwrap();// 转移所有权，留下 None
    println!("{}", stolen_text);
    println!("{:?}", s.text);
}