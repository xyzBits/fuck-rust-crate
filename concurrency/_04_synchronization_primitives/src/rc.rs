use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Rc 是 标准库中的一个智能指针，命名是 std::rc::Rc
/// 代表 reference counting ，它用于在多个地方共享数据时，通过引用计数来管理所有权
/// - Rc 通过引用计数来追踪指向数据的引用数量，当引用计数降为 0 时，数据会被自动释放
/// - Rc 允许多个 Rc 指针共享相同的数据，而无需担心所有权的转移
/// - Rc 内部存储的数据是不可变的，如果需要可变性，可以会用 RefCell Mutex 等内部可变性的机制
/// - Rc 在处理循环时需要额外注意，因为循环引用会导致引用计数无法降为 零，从而导致内存泄露
///       为了解决这个问题，可以使用 weak 类型
#[test]
fn test_rc_share_data() {
    let data = Rc::new(42);

    let reference_1 = Rc::clone(&data);
    let reference_2 = Rc::clone(&data);

    // 当 reference_1 reference_2 被丢弃时，引用计数减少
}

#[test]
fn test_rc_mut_data() {
    let shared_map = Rc::new(RefCell::new(HashMap::new()));

    {
        let mut map = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}
