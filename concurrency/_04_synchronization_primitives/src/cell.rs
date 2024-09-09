use std::cell::{Cell, RefCell};
use std::ops::Deref;

/// Cell RefCell 是 内部可变性两个重要类型
///
/// Cell RefCell 都是可共享的可变容器，可共享的可变容器的存在是为了以受控的方式允许可变性，即使存在引用别名
/// Cell RefCell 都允许在单线程环境下以这种方式进行，无论 Cell RefCell 都是线程不安全的，没有实现 Sync
///
/// Cell<T> 允许在不违反借用规则的前提下，修改其包含的值，Cell 中的值不再拥有所有权，只能通过 set get 方式访问，
/// set 可以在不获取可变引用的情况下修改 Cell 的值，适用于简单的单值容器，如整数或者字符
///
#[test]
fn test_cell() {
    // 创建一个 Cell，赋值给变量 x，注意，x 是不可变的，但是我们能够通过 set 修改它的值，并且即使存在对 x 的引用 y，也可以修改它的值
    let x = Cell::new(43);

    let y = &x;

    x.set(109);

    println!("y: {}", y.get());

    // let mut data = 4;
    // let data_ref = &data;
    // // cannot assign to `data` because it is borrowed
    // data = 4;
    // println!("{}", data_ref);
}

#[test]
fn test_ref_cell() {
    let x = RefCell::new(43);

    {
        // 在这个作用域内，只能获得不可变引用
        let y = x.borrow();
        let result = *y;
        println!("y: {}", result);
    }

    {
        // 在这个作用域内，可以获得可变引用
        let mut z = x.borrow_mut();
        *z = 998;
    }

    println!("x: {}", x.borrow().deref());
}
