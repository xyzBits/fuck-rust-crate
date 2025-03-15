/// Point 是一个 Sized 的类型，编译时明确知道 Point 的大小
///
/// DST 类型 dynamically sized types 不定长类型
/// 编译时大小不确定的业
/// 大小未知，编译器无法为它们分配明确或确定的内存布局，
/// 一种特殊的类型，老板娘通过间接方式，指针或者引用的方式来使用
/// 1. 切片 长度在编译时未知
/// 2. trait 对象 dyn trait，表示实现了某个trait 的类型，但具体类型和大小在编译时不确定
///
///
/// 对于 Sized 类型，编译器知道其大小，可以直接在栈上分配内存或者嵌入其他类型
/// DST 类型，大小未知，无法直接分配内存，必须通过额外的机制来处理
///
/// 通过胖指针来解决 DST 的问题，胖指针是一种包含额外信息的指针
/// 切片 [T] 一个指向数据的指针 + 一个长度
/// 对于 trait 对象 &dyn Trait： 一个指向数据的指针 + 一个虚表函数的指针
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_01() {
    let p = Point { x: 1, y: 2 };
    println!("size of point: {}", std::mem::size_of_val(&p));
}

#[test]
fn test_02() {
    let arr = [1, 2, 3];
    let slice: &[i32] = &arr;
    println!("Size of &i32: {}", std::mem::size_of::<&i32>()); // 8 字节 指针
    println!("Size of &[i32]: {}", std::mem::size_of_val(&slice)); // 16 字节 = 指针 + 长度
}



fn print_sized<T>(val: T) {
    println!("Size of T: {}", std::mem::size_of_val(&val));
}

#[test]
fn test_03() {
    print_sized(43);

    let data = [1, 2, 3];
    print_sized(&data[..]);
}