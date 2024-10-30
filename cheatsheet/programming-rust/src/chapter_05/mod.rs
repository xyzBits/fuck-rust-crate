use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

#[test]
fn test_1() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(table);

    table = Table::new();
    println!("{:?}", table);
}

/// 程序调用 table 时，整个结构体会被移动到函数时，变量 table 将变为未初始化
/// 正确的处理方式是使用引用，引用可以让你访问一个值，同时不影响它的的有权
/// 引用有两种：
/// -   共享引用，让你能读取，但不能修改引用的值，你可以同时拥有多个共享引用，
///     表达式 &e 返回一个指向 e 值的共享引用，如果 e 的类型是 T，那么 &e 的类型就是 &T，
///     读作 ref T，共享引用是 copy 类型
/// -   如果你有一个值的可变引用，你可以读取和修改这个值，然而，你不能再拥有任何其他有效的引用，
///     表达式 &mut e 返回一个指向 e 值的可变引用，它的类型是 &mut T，读作 ref mut T，可变引用不是 copy类型
///
/// 共享和可变引用可以看作是一种在编译期强制 多个读者，或者一个写者的方法
fn show(table: Table) {
    for (artist, works) in table {
        println!(" works by {}", artist);

        for work in works {
            println!("  {}", work);
        }
    }
}

#[test]
fn test_map_for() {
    let mut map = HashMap::new();
    map.insert("hello".to_string(), 33);

    // for 循环会 move 所有权
    for (key, value) in map {}

    // println!("{:?}", map);
}

#[test]
fn test_vec_for() {
    let data = vec![1; 4];

    // for 循环会 move 所有权
    for x in data {}
    // println!("{:?}", data);
}

fn show_v2(table: &Table) {
    // 迭代 hashmap 的共享引用被定义为 产生每个条目的 key value 的引用，artist 从一个 String 变化  &String
    // works 从 Vec<String> 变为 &Vec<String>
    for (artist, works) in table {
        println!(" works by {}", artist);

        // 内循环，迭代 vector 共享引用被定义为产生每个元素的共享引用，因此 work 是一个 &String，这个函数时不再有任何所有权的变化 ，
        // 而是传递各种无所有权的引用
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

#[test]
fn test_create_ref() {
    let x = 10;
    let r = &x;
    assert_eq!(10, *r);

    let mut y = 32;
    let m = &mut y; // 显式的解引用来访问 y 的值
    *m += 32;
    assert_eq!(64, *m);
}

#[test]
fn test_implicit_deref() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    };

    let aria = Anime {
        name: "Aria",
        bechdel_pass: true,
    };

    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria");

    // . 运算符会隐式地解引用它左侧的操作数
    assert_eq!((*anime_ref).name, "Aria");

    let mut data = vec![1, -3, 44, 22, 0];
    // 隐式借用 v 的可变引用
    data.sort();
    (&mut data).sort(); // 等价写法，不过更详细
    let iter = (&data).into_iter();
    println!("{:?}", data);
}
