use std::cell::{Cell, RefCell};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct Config {
    count: u32,
}

impl Config {
    fn new() -> Self {
        Config { count: 0 }
    }

    fn increment(&self) {
        // self.count += 1;// 错误，因为 self 是不可变引用，不能修改
    }
}
/// Cell<T>
/// 适合简单的值类型，比如数字，bool 等，这些类型必须是可拷贝的
/// 通过 set get 直接访问和修改值
/// 在单线程环境效率很高
struct CellConfig {
    count: Cell<u32>,
}

impl CellConfig {
    fn new() -> Self {
        CellConfig { count: Cell::new(0) }
    }

    fn increment(&self) {
        // 表面上不可变，内部却可以悄悄变化
        let current = self.count.get();// 获取当前值
        self.count.set(current + 1);// 设置新值
    }

    fn get_count(&self) -> u32 {
        self.count.get()
    }
}

#[test]
fn test_01() {
    let cell_config = CellConfig::new();
    cell_config.increment();// 增加计数器
    println!("Count: {}", cell_config.get_count());// 输出count: 1
}

/// RefCell<T>
/// 比 Cell 更灵活，适合需要引用的场景，比如字符串，向量等不可拷贝类型
/// 用 borrow 获取不可变引用，borrow_mut 获取可变引用
/// 在运行时检查借用规则，如果违反（比如同时有多个可变借用）会触发 panic
/// 也是单线程的
/// 运行时检查，如果在已经有一个可变借用时再次调用 borrow_mut()，或者在有不可变借用时尝试可变借用，程序会panic
struct RefCellConfig {
    count: RefCell<u32>,
    data: RefCell<Vec<u32>>,
}

impl RefCellConfig {
    fn new() -> Self {
        RefCellConfig { count: RefCell::new(0), data: RefCell::new(vec![0]) }
    }

    fn increment(&self) {
        let mut count = self.count.borrow_mut();// 获取可变借用
        *count += 1;

        let mut data = self.data.borrow_mut();
        (*data).push(1);
    }

    fn get_count(&self) -> u32 {
        let data = self.data.borrow();
        let len = data.len();
        println!("data len: {}", len);
        let count = self.count.borrow();// 获取不可变借用
        *count

    }
}

#[test]
fn test_02() {
    let config = RefCellConfig::new();
    config.increment();
    println!("Count: {}", config.get_count());
}

/// Mutex<T> RwLock<T>
/// 用于多线程环境，提供线程安全的内部可变性
/// Mutex 保证同一时间只有一个线程能够修改数据
/// RwLock 允许多个线程读取，但修改时需要独占访问
///
struct MutexConfig {

}

#[test]
fn test_03() {
    let config = Arc::new(RwLock::new("initial config".to_string()));

    let mut handles = vec![];

    for _ in 0..5 {
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::new(2, 0));
            let read_guard = config.read().unwrap();// 获取 读锁
            println!("read: {}", *read_guard);
        });
        handles.push(handle);
    }

    let config_write = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let mut write_guard = config_write.write().unwrap();// 获取写锁
        *write_guard = "updated config".to_string();
        println!("write: update config");
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final config: {}", *config.read().unwrap());
}



















































