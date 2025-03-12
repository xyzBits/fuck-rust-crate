use std::cell::RefCell;

struct User {
    name: String,
    age: u32,
}

#[test]
fn test_01() {
    let user = RefCell::new(
        User {
            name: "Alice".to_string(),
            age: 20,
        }
    );

    let mut borrow1 = user.borrow_mut();// 第一个可变借用
    borrow1.age = 31;

    let mut borrow2 = user.borrow_mut();// 尝试第二个可变借用
    borrow2.name = "Bob".to_string();// 这一行会导致 panic
}