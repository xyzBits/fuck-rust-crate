use std::rc::Rc;
use std::thread::ThreadId;

#[test]
fn test_01() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    // let u = s;
}

#[test]
fn test_02() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];

    let t = s.clone();
    let u = s.clone();
}

#[test]
fn test_03() {
    let mut s = "Govida".to_string();
    s = "Siddhartha".to_string(); // 值  Govida 在这里 drop
}

#[test]
fn test_04() {
    let mut s = "Google".to_string();
    let t = s; // t 拿走了 s 中原本字符串的所有权，因此 当我们给 s 赋值时，它是未初始化的，这种场景下，不会发生 drop
    s = "Apple".to_string(); // 这里不会 drop 任何内容
}

// 这里使用的初始化和赋值的例子足够简单，但是rust几乎在所有场景下都使用 move
// 向函数传参会把所有权移动给函数的参数
// 从函数返回值会把所有权移动给调用者，创建一个元组会把值移动给元组
#[test]
fn test_05() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = vec![];
    let person = Person {
        name: "Palestrina".to_string(), // 结构体获得了字符串的所有权
        birth: 1525,
    };
    composers.push(person); // 整个 person 结构体，而不是指针，被传递给 vector 的push方法，vector 获得了 person 的所有权，因此 也变成了 name string的间接所有者
}

#[test]
fn test_06() {
    fn hx(input: Vec<i32>) {}
    fn gx(input: Vec<i32>) {}

    let data = vec![1, 2, 3];
    let switch = true;
    if switch {
        hx(data);
    } else {
        gx(data);
    }

    // 如果一个变量的值被移动走，并且从此之后没有再被赋予一个新的值，那么它被认为是未初始化的，
    // println!("{:?}", data);
}

#[test]
fn test_07() {
    let mut data = vec![1, 2, 3];
    fn fx() -> bool {
        true
    }
    fn gx(input: Vec<i32>) {}
    fn hx() -> Vec<i32> {
        vec![]
    }
    while fx() {
        // gx(data);
        // data 会在第一次迭代时被移动，
        // 第二次迭代时就是未初始化状态

        gx(data); // 移动x 的值
        data = hx(); // 给 x 一个新值
    }
}

#[test]
fn test_08() {
    let mut data = vec![];
    for i in 101..106 {
        data.push(i.to_string());
    }

    // 从 vector 中随机取出元素
    // let third = data[2];// 不能移动vec索引
    // let fifth = data[4];
    // 如果上面的代码要工作，rust 需要记住 第3 和 5 元素变成了未初始化的状态，然后一直追踪这些信息，直到vector 被 drop

    // 建议使用 引用
    let third = &data[2];
    println!("{}", third);
}

#[test]
fn test_09() {
    // 如果真的想要移除一个元素呢
    let mut data = vec![];

    for i in 101..106 {
        data.push(i.to_string());
    }

    // 1。 弹出 vector 尾部的元素
    let fifth = data.pop().expect("vector empty");
    assert_eq!(fifth, "105");

    // 2。 移出给定位置的元素，并把最后一个元素移动过来
    let second = data.swap_remove(1);
    assert_eq!(second, "102");

    // 3。 用另一个值和我们想要移出的值交换
    let third = std::mem::replace(&mut data[2], String::from("substitute"));
    assert_eq!(third, "103");

    println!("{:?}", data);
}

#[test]
fn test_10() {
    let v = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

#[test]
fn test_11() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = vec![];

    let person = Person {
        name: Some("hello".to_string()),
        birth: 1525,
    };

    composers.push(person);

    let first_name = std::mem::replace(&mut composers[0].name, None);
    println!("{:?}", composers);
    println!("{}", first_name.unwrap());
}

#[test]
fn test_12() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = vec![];

    let person = Person {
        name: Some("hello".to_string()),
        birth: 1525,
    };

    composers.push(person);

    // take 里面也是调用 std::mem::replace
    let first_name = composers[0].name.take();
    println!("{:?}", composers);
    println!("{}", first_name.unwrap());
}

#[test]
fn test_13() {
    #[derive(Debug)]
    struct Label {
        number: i32,
    }

    fn print_label(label: Label) {
        println!("{:?}", label);
    }

    let label = Label { number: 1 };
    print_label(label);

    // 默认情况下 struct enum 都是 move类型
    // println!("{:?}", label);
}

#[test]
fn test_14() {
    let s = Rc::new("shirataki".to_string());

    let t = s.clone();
    let u = s.clone();

    // 可以直接对 Rc<String> 使用任何 String 的方法
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quiet chewy, almost bouncy, but lack flavor", u);
}
