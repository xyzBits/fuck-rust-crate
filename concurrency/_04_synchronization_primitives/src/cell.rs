use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::result;

#[test]
fn test_cell() {
    let x = Cell::new(43);

    let y = &x;

    x.set(109);

    println!("y: {}", y.get());
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
