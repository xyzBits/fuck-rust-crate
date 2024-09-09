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
