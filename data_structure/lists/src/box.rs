#[cfg(test)]
#[allow(dead_code)]
#[allow(unused)]
mod box_test {
    use std::cell::{Cell, RefCell};
    use std::ops::{Deref, DerefMut};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_box_1() {
        // 在堆上分配一个 i32 类型的值，然后用栈帧里的 b 变量来存放它的地址
        let b = Box::<i32>::new(1);

        // 通过解引用操作来获取 Box 中的值
        let k = b.deref();

        println!("k = {}", k);

        assert_eq!(*k, 1);
    }

    #[test]
    fn test_box_2() {
        let b = Box::<i32>::new(1);
        let mut k = b.deref();

        // 这里虽然改变了 k，但 k 是一个引用，堆上的值并没有改变
        k = &2;
        println!("k = {k}");
        println!("b = {b}");
    }

    #[test]
    fn test_box_3() {
        let mut b = Box::new(1);
        let mut k = b.deref_mut();

        // 试图改变堆上的值
        *k = 2;
        println!("{}", k);
        println!("{}", b);
    }


    #[test]
    fn test_box_4() {
        let mut b = Box::new(1);
        *b = 3;
        println!("{}", b);
    }


    struct S<'x_lt> {
        x: &'x_lt mut i32,
    }

    #[test]
    fn test_interior_mutate() {
        let mut x = 5;
        // 虽然 S 是不可变的，但是它的字段 x 是可变的，这种可变是内部性的
        let s = S { x: &mut x };

        // 如果在修改 s 内部的 x 之前，加入对 x 的修改，会编译失败
        // x += 1;
        // incr x by 1
        *s.x += 1;

        x += 1;

        // RUST 中的原则 ，可以同时拥有一个值的不可变引用，但是只能有一个可变引用
        // 当 x += 1; 在中间时，持有可变引用的是 s，因此不能再进行修改，否则会造成两个可变引用同时存在
        // 当 x += 1; 在后面时，s 看似持有可变引用，但没有修改 s.x 的意图，因此不会造成两个可变引用现时存在
        // 那么，什么时候可以同时持有两个可变引用呢，这就需要用到cell
    }

    /// Cell 是在栈帧中得到一个内存单元，然后用这个内存单元存放一个值 T，只是编译器层面的一个结束
    /// Cell 的生命周期由 栈帧管理
    /// Cell 本身是不可变的，它存放的值 T 可变的，
    /// get 只能得到值的副本，因此 T 必须实现Copy,
    /// 使用 get_mut 则可以得到值的可变引用
    #[test]
    fn test_cell_1() {
        let c = Cell::new(1);
        let k = c.get();
        println!("k = {k}");
        assert_eq!(k, 1);
    }

    #[test]
    fn test_cell_2() {
        let mut c = Cell::new(1);
        c.set(2);

        assert_eq!(c.get(), 2);

        *c.get_mut() += 1;
        assert_eq!(c.get(), 3);

        let x = c.get_mut();
        let i = c.get();
    }


    // RefCell 几乎和 Cell 是样，区别是：
    // 1. RefCell<T> 提供引用，而 Cell<T> 提供值，会产生copy
    // 2. Cell 通过 set/get 来访问值，RefCell 通过 borrow borrow_mut 并结合引用等方法来访问值
    // 3. RefCell<T> 可能引起 panic，而 Cell<T>则不会
    // 内部可变性，使得我们可以把一个可变的值放在一个不可变的容器中，而在工程实践中，
    // RefCell 的主要作用是，允许我们在同一作用域中多次对同一值进行更改
    #[test]
    fn test_ref_cell_1() {
        let c = RefCell::new(1);
        *c.borrow_mut() += 1;
        *c.borrow_mut() += 1;
        assert_eq!(*c.borrow_mut(), 3);

        let x = c.borrow();
    }


    #[test]
    fn test_ref_cell_2() {
        let c = RefCell::new(1);
        let k = c.borrow_mut();
        // k 的生命周期是由 c 管理
        //
        let k2 = &c;

        let ref a = 2;
        let b = &3;
    }


    #[test]
    fn test_rc_1() {
        let a = Rc::new(1);
        println!("rc = {}", Rc::strong_count(&a));
        {
            let b = a.clone();
            println!("rc b = {}", Rc::strong_count(&b));
            {
                let c = a.clone();
                println!("rc b = {}", Rc::strong_count(&c));
                {
                    let d = a.clone();
                    println!("rc d = {}", Rc::strong_count(&d));
                }
            }
        }
    }

    /// 由于 Rust 的 mutable 特性，一个结构休中的字段，要么全部是 immutable，要么全部是 mutable
    /// 不支持针对部分字段进行设置，比如一个 struct 中，可能只有个别的字段需要修改，而其他字段不需要修改，
    /// 为了一个字段而将整个 struct 变为 &mut 也是不合理的
    /// 所以，实现内部可变性的 Cell RefCell 正是为了解决此类问题存在的，通过它们可以实现 struct 部分字段可变
    /// 而不用将整个 struct 设置为 mutable
    #[test]
    fn test_rc_ref_cell() {
        // 让三个作用域共享一个 Rc<T> ，并且各自修改其中的值，打印引用计数和值
        let a = Rc::new(RefCell::new(0));
        *a.borrow_mut() += 1;
        println!("rc = {}, a = {}", Rc::strong_count(&a), *(*a).borrow());
        {
            let b = a.clone();
            *b.borrow_mut() += 1;
            println!("rc = {}, b = {}", Rc::strong_count(&b), *(*b).borrow());

            {
                let c = b.clone();
                *c.borrow_mut() += 1;
                println!("rc = {}, a = {}, b = {}. c = {}",
                         Rc::strong_count(&b),
                         *(*a).borrow(),
                         *(*b).borrow(),
                         *(*c).borrow());
            }
        }
    }

    #[test]
    fn test_arc_mutex() {
        let data = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        for _ in 0..10 {

            let data = data.clone();
            let handle = thread::spawn(move || {
                let mut data = data.lock().unwrap();
                *data += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let data = data.lock().unwrap();
        println!("result = {}", *data);
        assert_eq!(*data, 10);
    }
}