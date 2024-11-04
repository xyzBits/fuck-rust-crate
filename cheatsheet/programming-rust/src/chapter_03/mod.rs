use std::fmt::Display;
use std::vec;

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

#[test]
fn test_05() {
    assert_eq!('*' as i32, 42);
    println!("{}", 'a' as u8);
    assert_eq!('a' as u8, 97);

    assert_eq!('*'.is_alphanumeric(), false);
    assert_eq!('b'.is_alphanumeric(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

#[test]
fn test_06() {
    let text = "I see the eigenvalue in the thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in the thine eye");
}

// &T 一个不可变的共享引用，你可以拥有同一个值的多个共享引用，但它们都是只读的，修改它们的指向是禁止的，就像 C 中的 const T * 一样
// &mut R 可变的，独占的引用，可能读写它的值，类似于 T* ，只有这个引用存在，不能再持有其他任何类型的引用
// 事实上，这时候你唯一可以访问这个值 的方法就是通过这个可变引用

// &[T] &mut[T] 把切片当作指向第一个元素的指针 + 一个可访问的元素数量
// 一个可变切换 &mut[T] 让你可以读取并修改元素，但不能被共享
#[test]
fn test_07() {
    let lazy_caterer = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
}

#[test]
fn test_08() {
    let mut sieve = [true; 10_000];

    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10_000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert_eq!(sieve[211], true);
    assert_eq!(sieve[9876], false);
    // 这种语法还可以用来创建固定大小的缓冲区，[0u8; 1024] 可以用作 1k 的缓冲区，所有字节全部初始化为 0
}

// 数组有一些方法，迭代元素，搜索，排序，填充，过滤等，这些方法都是切片的，
// 但在使用时，rust 会隐式的把数组的引用转为切片，因此 你可以直接调用数组切片的方法
#[test]
fn test_09() {
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_10() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let data: Vec<i32> = vec![1, 2, 3, 4];
    let product = data.iter().product::<i32>();
    assert_eq!(product, 24);
}

#[test]
fn test_11() {
    // 通过迭代器创建
    // 通常使用 collect 时你需要给出类型，因为它可以生成不同类型的集合，不只是 vector
    let v = (0..5).collect::<Vec<_>>();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_12() {
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man",]);
}

#[test]
fn test_13() {
    // len 方法返回持有的元素数量
    // capacity 返回不重新分配的前提下能存储的最大元素数量
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());
}

#[test]
fn test_14() {
    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);

    assert_eq!(v, vec![10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, vec![10, 30, 35, 40, 50]);
}

#[test]
fn test_15() {
    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);
}

// 一个切片写作  [T] 没有长度，表示数组或者 vector 的一部分，因为切片可以是任意长度，
// 因此切片不能直接存储在变量中或者作为参数传递，切片必须 通过引用传递
// 切片的引用是胖指针，一个包含指向切片中第一个元素的指针和切片中元素数量的双字值
// 一个普通的引用是一个指向单个值的无所有权的指针
// 而一个切片的引用是指向内存中连续范围的指针，如果在处理数组或者vector时，切片引用是一个很好的选择
fn print_slice<T: Display>(inputs: &[T]) {
    inputs.iter().for_each(|x| println!("{}", x));
}

#[test]
fn test_16() {
    let v = vec![0.0, 0.707, 1.0, 0.707];
    let a = vec![0.0, 0.707, 1.0, 0.707];

    // 自动将 &Vec<f64> &[f64; 4] 转换为 &[f64] 切片
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print_slice(&v); // 可以用于vector
    print_slice(&a); // 可以用于数组

    print_slice(&v[..2]);
    print_slice(&a[2..]);
    print_slice(&sv[1..3]);
    // 尝试超过数据结尾会导致 panic
}

// &str 就相当于 &[u8]
#[test]
fn test_17() {
    let mut data = vec![9, 1, 2, 3, 4];
    print_slice(&data);

    // 可以使用可变切片引用进行一些排序，逆序的操作，但是无法 添加 或者 删除元素
    let slice_data = &mut data[..2];
    slice_data.sort();
    slice_data.reverse();
    println!("{:?}", data);
}
