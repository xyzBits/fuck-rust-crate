use std::sync::{Arc, Mutex};
use std::thread;

/// Arc 代表原子引用计数器，是一种用于多线程环境的智能指针，
/// 允许在多个地方共享数据，同时确保线程安全，是标准库的一部分
///
/// 在 Rust 中，通常情况下，变量是被所有权管理的，但有时候我们需要在多个地方共享数据
/// 这就是 Arc 的用处，
/// 它通过在 堆上分配内存，并使用引用计数来跟踪数据的所有者数量，确保在不需要的时候正确的释放资源
///
/// Arc 原子引用计数
/// Rc  引用计数
/// 都是 Rust 中用于所有权的智能指针，但是有一些关键的区别
/// 线程安全：
///     - Arc 是线程安全的，可以安全地在多线程环境中共享，它使用原子操作来更新引用计数，确保并发访问时的线程安全性
///     - Rc  不是线程安全的，它只适用于单线程环境，因为它的引用计数操作不是原子的，可以导致在多线程中的竞态条件和不安全行为
/// 性能开销：
///     - 由于 Arc 使用原子操作来更新引用计数，相对于 Rc Arc 的性能开销更大，原子操作通常比非原子操作更昂贵
///     - Rc 在单线程环境中性能更好，因为它不需要进行原子操作
/// 可变性：
///     - Arc 不能用于可变数据，如果需要在多线程环境中共享可变数据，通常会使用 Mutex RwLock 等同步原语和 Arc
///     - Rc 也不能用于可变数据，因为它无法提供并发访问的安全性
/// 引用计数减少时的行为：
///     - 当 Arc 的引用计数减少为 零时，由于它是原子的，它会正确的释放底层资源，比如堆上的数据
///     - Rc 在单线程中引用计数减少为零时会正确释放资源，但在多线程中可能存在问题，因为它没有考虑并发情况
///
/// ------------------------在多线程情况下使用 Arc，在单线程情况下使用 Rc -------------------------------------
///
///
#[test]
fn test_arc_example_1() {
    // 创建一个可共享的整数
    let data = Arc::new(46);

    // 创建两个线程，共享对 data 的引用
    let thread1 = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            // 在线程中使用 data
            println!("Thread 1: {:?}", data);
        })
    };

    let thread2 = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            // 在另一个线程中使用 data
            println!("Thread 2: {:?}", data);
        })
    };

    // 等待两个线程完成
    thread1.join().unwrap();
    thread2.join().unwrap();
}