use std::fmt::Display;
use std::thread;

/// &'static 是一种引用类型，生命周期被标记为 'static
/// 生命周期是编译器为了追踪引用有效性的概念
/// 'static 是特征的生命周期，
/// 引用的数据在程序启动时就存在，并在整个程序运行期间都不会被释放
///
/// &'static 引用的数据是静态的，生存时间和程序的生命周期一样
/// 字符串字面量，"hello" 类型是 &'static str
/// 字符串在编译时就被嵌入二进制文件中，程序运行时一直存在，因此生命周期是 &'static
#[test]
fn test_01() {
    let mark_twain = "Samuel Clements";
    print_author(mark_twain);

    let data: &'static str = "hello world";
}

fn print_author(author: &'static str) {
    println!("{}", author);
}

/// Foo 的生命周期取决于 data 引用的生命周期 'a
/// 如果我们要求 Foo 满足 T: 'static
/// 那么 'a 必须是 'static
/// data必须 是 &'static str
struct Foo<'a> {
    data: &'a str, // 'a 是一个生命周期参数
}

fn print_static<T: 'static>(_value: T) {
    // println!("value: {}", _value);
}

#[test]
fn test_02() {
    let static_str = "hello";
    let foo = Foo { data: static_str };
    print_static(foo);
}

// 静态数据，满足 'static
static GLOBAL_NUM: i32 = 42;
#[test]
fn test_03() {
    // Vec<&'static i32> 满足 T: 'static
    let data: Vec<&i32> = vec![&GLOBAL_NUM];
    process_data(data);
}

fn process_data<T: 'static>(_: T) {}

#[test]
fn test_04() {
    let local_num = 42;
    let v: Vec<&i32> = vec![&local_num];

    // `local_num` does not live long enough
    // process_data(v);
}

/// T: 'static 的真正含义，要么数据完全自包含，无引用，
/// 要么内部的引用都是静态的
/// T: 'static != 数据永远存在，只表示数据不依赖外部生命周期，而非必须存活到程序结束
/// String 类型可以被提前释放，但仍满足 'static 约束
struct Container<'a> {
    data: &'a i32, // 包含引用
}

#[test]
fn test_05() {
    let num = 42;
    let c = Container { data: &num };
    // process_data(c);
}

#[test]
fn test_06() {
    let c = Container { data: &GLOBAL_NUM };

    process_data(c);
}

fn spawn_thread<T: Send + 'static + Display>(data: T) {
    thread::spawn(move || {
        println!("Data: {}", data);
    });
}

#[test]
fn test_07() {
    let num = 42; // i32 是 'static
    spawn_thread(num);

    let local_str = "hello".to_string();
    spawn_thread(local_str); // String 是 'static

    let invalid = &num; // &i32 不是 'static
                        // spawn_thread(invalid);
}
