use std::ascii::AsciiExt;
use std::borrow::Cow;

/// clone on write
/// copy on write
/// copy on write 是一种优化内存和提高性能的技术，通常应用在资源共享的场景
/// 其基本思想是，当有多个调用者（callers） 同时请求相同的资源时，
/// 都会共享同一份资源，直到有调用者修改资源内容时，系统才会真正复制一份副本出来给调用者，
/// 而其他调用者仍然使用原来的资源
///
/// Rust 中的 String Vec 等类型就利用了 COW
#[test]
fn test_cow_1() {
    let s1 = String::from("hello");
    let mut s2 = s1; // s1 s2 共享同一份内存

    s2.push_str(", world!"); // s2 会进行写操作，于是系统复制一份新的内存给 s2
}

/// 这样可以避免大量未修改的字符串，向量等的重复分配和复制，提高内存利用率和性能
/// cow 的优点是：- 内存利用率高，只有进行写时才复制，读取性能高，多个调用者共享同一资源
/// 缺点是：写时需要复制，有一定性能损失，实现较复杂
///
/// 需要根据实际场景权衡使用，但对于大量相同或者相似资源共享的情况，使用 cow 可以带来显著的性能提升
///
/// 标准库中的 std::borrow:Cow 是一个智能指针，提供了写时克隆的功能，
/// 它可以封装并提供对借用数据的不可变访问，当需要进行修改或获取所有权时，它可以惰性地克隆数据
///
/// Cow 实现了 deref 你可以直接在其封装的数据上调用不可变方法，
/// 如果需要进行改变，则 to_mut 将获取一个对拥有的值 的可变引用，必须时进行克隆
///
#[test]
fn test_cow_2() {
    let origin = "hello world";
    let mut cow = Cow::from(origin);

    assert_eq!(cow, "hello world");

    let s: &str = &cow;
    assert_eq!(s, "hello world");

    assert_eq!(s.len(), cow.len());

    let s: String = cow.to_uppercase().into();
    assert_eq!(s, "HELLO WORLD");
}

#[test]
fn test_cow_clone() {
    let origin = "hello world";
    let mut cow = Cow::from(origin);

    // 要克隆的数据上进行修改
    let s: &mut str = cow.to_mut();
    s.make_ascii_uppercase();

    assert_eq!(s, "HELLO WORLD");
    assert_eq!(origin, "hello world");
}

/// 将字符串的一部分进行替换，返回值类型是 String
/// 意味着 每次函数调用都会分配新的内存空间来存储结果
fn change_string(input: &str) -> String {
    if input.contains("Java") {
        input.replace("Java", "Rust")
    } else {
        input.to_string()
    }
}

/// 只有当字符串中包含 Java 时，才会创建新的内存空间来存储替换后的结果，
/// 其他情况下， 函数会直接返回对原字符串的引用，从而避免了内存分配的开销
///
/// Cow 通常适用于以下场景：
/// - 函数返回值：当函数返回值可能要修改原始数据，也可能不修改原始数据，可以使用 Cow 来避免不必要的克隆操作
/// - 数据结构： 在一些数据结构中，如 vector hashMap 可以使用 Cow 来实现高效的数据共享
///
///
/// Cow 的优势：
/// - 性能提升：避免不必要的克隆操作，Cow 可以有效地提高程序的性能，尤其是在处理大型数据结构时
/// - 内存优化：Cow 可以减少程序的内存占用，因为它只在必要时才分配新的内存空间
///
/// Cow::Borrowed 表示对数据的引用，不进行任何克隆操作
/// Cow::Owned 表示拥有数据的所有权，并在内存存储数据的副本
fn change_string_with_cow(input: &str) -> Cow<str> {
    if input.contains("Java") {
        let replaced = input.replace("Java", "Rust");
        Cow::Owned(replaced)
    } else {
        Cow::Borrowed(input)
    }
}

#[test]
fn test_change_string() {
    let borrowed_str = "I love Java";
    let changed_str = change_string(borrowed_str);

    println!("{}", changed_str);
}

fn concat_strings<'a>(s1: &'a str, s2: &'a str) -> Cow<'a, str> {
    if s1.is_empty() {
        Cow::Borrowed(s2)
    } else if s2.is_empty() {
        Cow::Borrowed(s1)
    } else {
        let mut result = String::from(s1);
        result.push_str(s2);
        Cow::Owned(result)
    }
}

#[test]
fn test_concat_strings() {
    let str1 = "Hello, ";
    let str2 = "world!";

    let result1 = concat_strings(str1, str2);
    println!("{}", result1);

    let str3 = "";
    let result2 = concat_strings(str1, str3);

    println!("{}", result2);
}
