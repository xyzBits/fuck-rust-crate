// rust 中所有都是表达式，都能产生值
// 控制流也是表达式
#[test]
fn test_01() {
    let messages = vec!["hello".to_string(), "world".to_string()];

    // for 循环会消耗掉值
    for msg in messages {
        println!("{}", msg);
    } // messages 在这里就 drop 了
}

#[test]
fn test_02() {
    let messages = vec!["hello".to_string(), "world".to_string()];

    for msg in &messages {
        println!("{}", msg);
    }

    println!("{}", messages.len());
}

#[test]
fn test_03() {
    let mut messages = vec!["hello".to_string(), "world".to_string()];

    for msg in &mut messages {
        msg.push_str(" !");
    }

    println!("{}", messages.len());
}

// rust 中的 break 只能在循环中使用，match 表达式中不需要它，这一点和 c 中的 switch 不同
// 函数并不需要电显式的 return 表达式，函数体就像是一个块表达式，如果最后一个表达式后面没有分号，它的值将是函数的返回值
// 事实上，这是rust中充当返回值的最佳方法

// 取地址运算符 & &mut
// 一元 * 运算符用于获取引用指向的值，
// 正如我们看到的，当你使用 . 运算符访问字段或者方法时
// Rust 会自动解引用，因此只有我们想读取或写入引用指向的整个值时，* 才是必须的
#[test]
fn test_04() {
    let nums = vec![1, 2, 3, 4];
    for num in &nums {
        // 知道引用指向的某个值时
        let data = *num;
    }
}

#[test]
fn test_05() {
    let x = 17;
    let index = x as usize;
    // bool char enum 类型可以转换为任意整数类型
    // 其他类型转为 bool char enum 是不允许的
    let _x = true as usize;
    let _x = '2' as i64;
    enum Gender {
        Male,
        Female,
    }

    let _x = Gender::Male as usize;

    // 无法转换
    // let _x = 9 as bool;

    // u8 可以转为 char
    let x1 = 43 as char;
    println!("x1 = {}", x1);
}

#[test]
fn test_06() {
    // &String &tr
    // &Vec<i32> &[i32]
    // &Box<Chessboard> &Chessboard
}

#[test]
fn test_07() {
    let is_even = |x| x % 2 == 0;

    assert_eq!(is_even(8), true);

    let is_odd = |x: i64| x % 2 != 0;
    assert_eq!(is_odd(5), true);
}


#[test]
fn test_08() {
    let name = "123".to_owned();
    let opt: Option<i32> = Some(1);

    let step_1 = opt.map(|_| name);

    // if let Some(name) = step_1 {
    //     println!("{}", name);
    // } else {
    //     let _closure = || name;
    // }

    // let string = opt
    //     .map(|_| name)
    //     .unwrap_or_else(|| "Hello World".to_owned());
    //
    // println!("{}", string);
}