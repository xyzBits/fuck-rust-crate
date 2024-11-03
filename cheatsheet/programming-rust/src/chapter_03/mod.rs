fn build_vector() -> Vec<i16> {
    let mut v = vec![];

    v.push(10);
    v.push(20);

    v
}

// rust 使用 u8 表示一个字节的值，例如，从二进制文件中读取或者socket 中读取数据就会返回 u8 类型的流数据

#[test]
fn test_01() {
    // 在类型表示的范围内进行 转换
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    println!("{}", 65535_u16 as i32);

    // 超出范围的转换
    // 等价于原值对 2^N 取模
    // N 是目标类型的位数
    assert_eq!(1000_i16 as u8, 232_u8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b_101_101_u8.count_ones(), 4); // 二进制 中 0 1 的个数

    // 解析 abs 方法时，需要知道 -4 的类型
    // println!("{}", (-4).abs());、
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}

#[test]
fn test_02() {
    let mut i = 1;
    loop {
        i *= 10;
    }
}

#[test]
fn test_03() {
    let mut i = 1_i8;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

#[test]
fn test_04() {
    assert_eq!(true as u8, 1);
    assert_eq!(false as u8, 0);
}
