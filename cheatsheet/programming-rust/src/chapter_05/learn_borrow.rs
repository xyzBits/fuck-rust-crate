use std::cell::RefCell;

struct User {
    name: String,
    age: u32,
}

#[test]
fn test_01() {
    let user = RefCell::new(User {
        name: "Alice".to_string(),
        age: 20,
    });

    let mut borrow1 = user.borrow_mut(); // 第一个可变借用
    borrow1.age = 31;

    let mut borrow2 = user.borrow_mut(); // 尝试第二个可变借用
    borrow2.name = "Bob".to_string(); // 这一行会导致 panic
}

#[test]
fn test_02() {
    let mut b = Box::new(5);

    {
        // * 得到box 内部的数据，& 拿到引用
        // deref 把 &Box<T> 转为 &T
        // DerefMut 把&mut Box<T> 转为&mut T
        let borrow = &*b; // 不可变借用
        println!("{}", borrow);

        let borrow2: &i32 = &b;
        println!("{}", borrow2);
    } // 在此处释放

    {
        let borrow_mut = &mut *b;
        *borrow_mut = 6;
    } // 在此处释放

    println!("{}", b); // box 仍然可用
}
