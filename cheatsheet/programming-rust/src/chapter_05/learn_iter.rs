#[test]
fn test_01() {
    let mut numbers = vec![1, 2, 3];

    for num in numbers.iter() {
        println!("immutable ref: {}", num);
    }

    for num in numbers.iter_mut() {
        *num += 1;
    }
    println!("after updated {:?}", numbers);

    for num in numbers.into_iter() {
        println!("owner num: {}", num);
    }

    // numbers 在此被消耗，无法再次使用
}

struct Counter {
    count: u32, // 当前计数
    max: u32,   // 上限
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_02() {
    let mut counter = Counter::new(10);

    for num in counter {
        println!("{}", num);
    }
}

#[test]
fn test_03() {
    let numbers = vec![1, 2, 3, 4, 5];
    let first_three = numbers.iter().take(3).collect::<Vec<_>>();
    println!("{:?}", first_three);

    // let numbers = vec![1, 2, 3, 4, 5];
    let after_first_two = numbers.iter().skip(2).collect::<Vec<_>>();
    println!("{:?}", after_first_two);

    let fruits = vec!["apple", "banana", "cherry"];
    // enumerate 为迭代器中的每一个元素添加一个索引，生成一个新的迭代器，返回 (index, value) 元组，当你需要同时访问元素的索引和值时
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", i, fruit);
    }
}

#[test]
fn test_04() {
    let words = vec!["hello", "world"];
    let chars = words
        .iter()
        // 对每个单词调用 chars(返回字符迭代器)，然后将所有的字符迭代器展平成一个单一的字符迭代器
        .flat_map(|word| word.chars())
        .collect::<Vec<_>>();
    println!("{:?}", chars);
}

#[test]
fn test_05() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flattened: Vec<_> = nested.iter().flatten().collect();

    println!("{:?}", flattened);

    let flat_map: Vec<_> = nested.iter().flat_map(|item| item.iter()).collect();
    println!("{:?}", flat_map);
}

#[test]
fn test_06() {
    let data = vec![Some(1), None, Some(2), Some(3)];

    let flat_mapped: Vec<_> = data
        .iter()
        .flat_map(|opt| {
            let option = opt.map(|i| i * 2);
            option
        })
        .collect();
    println!("{:?}", flat_mapped);
}
