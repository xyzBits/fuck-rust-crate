use std::sync::{Arc, Mutex};
use std::thread;

/// 当你需要在多线程环境中共享可变数据时，常常会结合使用 Arc Mutex
/// Mutex 是互斥锁，用于确保在任意时刻只有一个线程能够访问被锁定的数据，
///
/// mutex 提供了一种机制，只有拥有锁的线程才能访问被锁定的数据，其他的线程必须等待锁的释放
///
/// counter 是一个 mutex 保护的可变整数，然后使用 Arc 来多线程共享，
/// 在每个线程中，通过 counter.lock().unwrap() 来获取锁，确保一次只有一个线程能够修改计数器的值，
/// 这样可以确保在并发情况下不会发生竞态条件
///
/// lock 方法返回的是一个 MutexGuard ，它是一个智能指针，实现了 Deref Drop trait
/// 当 MutexGuard 被销毁时，会自动释放锁，确保在任何情况下能够正确的释放锁
///
/// 为了跨线程支持，一般 Mutex Arc 组合使用，这样 Mutex 对象在每个线程中都能安全访问
/// lock 方法返回实现了 Deref trait 的 MutexGuard
/// 所以会自动解引用，可以直接调用被保护对象上的方法
///
#[test]
fn test_arc_mutex() {
    // 创建一个可共享的 可变数据
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取锁，确保只有一个线程能够访问计数器
            // 其他线程必须等待锁的释放
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("Final counter: {}", *counter.lock().unwrap());
}

#[test]
fn test_mutex_try_lock() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 尝试获取锁，如果获取失败就继续尝试或者放弃
            if let Ok(mut num) = counter.try_lock() {
                *num += 1;
            } else {
                println!("Thread failed to acquire lock");
            }

            // 这里好像可以重入
            // let mut guard = counter.lock().unwrap();
            // *guard += 1;
        });

        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Final counter: {}", *counter.lock().unwrap());
}
