use std::rc::Rc;
use std::sync::Arc;
use std::thread;

/// Send
/// 如果一个类型 T 实现了 Send，意味着 T 的值可以安全地从一个线程移动到另一个线程
/// 实现了 Send 的类型可以在线程之间传递所有权，而不会导致未定义的行为
///
/// Sync
/// 如果一个类型 T 实现了 Sync，意味着 &T，即共享引用可以安全地被多个线程同时访问
/// 可以通过共享引用 &T 被多个线程安全的访问，而不会引发数据竞争
/// T 是 Sync 的等价于 &T 是 Send 的
#[test]
fn test_01() {
    // 当你需要将数据的所有权转移给另一个线程时，编译器会检查类型是否实现了 Send
    let value = 42; // i32 实现了 Send
    thread::spawn(move || {
        println!("Received: {}", value);
    });
}

#[test]
fn test_02() {
    // 当你需要在多个线程中共享数据的引用时，编译器会检查类型是否实现了 Sync，例如，通过 Arc 共享数据
    // Rc 没有实现 Send Sync，因此不能在向其他线程移动
    let data = Rc::new(42);
    let data_clone = data.clone();
    // Rc<i32> cannot be sent between threads safely
    // thread::spawn(move || {
    //    println!("Received: {}", data_clone);
    // });
}

#[test]
fn test_03() {
    // Arc::clone() 和 data.clone() 是等价的
    let data = Arc::new(42);
    let data_clone = Arc::clone(&data);
    let another = data_clone.clone();

    thread::spawn(move || {
        println!("Received: {}", data_clone);
    });
    let other = data.clone();
    let count = Arc::strong_count(&data);
    println!("{}", count);
}

///Rc<i32> 不是 Sync
// Rc 使用非原子的引用计数器，设计用于单线程环境。如果多个线程同时访问并修改 Rc 的引用计数，会导致数据竞争，因此 Rc<i32> 没有实现 Sync。
//
// Arc<Rc<i32>> 也不是 Sync
// Arc<T> 只有在 T 实现 Sync 时才会实现 Sync。因为 Rc<i32> 不是 Sync，所以 Arc<Rc<i32>> 也不满足 Sync 的要求。
//
// 线程安全性要求
// thread::spawn 要求闭包是 Send 的，以便安全地将数据移动到新线程。data_clone 是 Arc<Rc<i32>> 类型，但由于 Arc<Rc<i32>> 不是 Sync，它的引用 &Arc<Rc<i32>> 不是 Send，导致闭包无法满足 Send 要求，编译器拒绝编译。
#[test]
fn test_04() {
    // let data = Arc::new(Rc::new(42));
    // let data_clone = &data.clone();
    //
    // let handle = thread::spawn(move || {
    //     println!("Received: {}", data_clone);
    // });
    //
    // println!("main received: {}", **data);
    // handle.join().unwrap();
}

#[test]
fn test_05() {
    let data = "hello world";

    let v = [1, 2, 3];
}
