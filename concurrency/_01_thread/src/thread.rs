use std::thread;
use std::time::Duration;

#[test]
fn test_start_one_thread() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    // 如果注释掉了下面，可能文本不会被输出，原因是当主程序退出时，即使这些新开的线程也会被退出，
    // 所以需要通过 join 等待线程完成，调用者不得不等待线程执行完成
    handle.join().unwrap();
}

#[test]
fn test_start_one_thread_result() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
        200
    });

    match handle.join() {
        Ok(result) => {
            println!("thread result: {}", result);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}

#[test]
fn test_start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn test_start_n_threads() {
    const N_THREADS: isize = 10;

    let handles = (0..N_THREADS)
        .map(|i| {
            // may outlive borrowed value i
            thread::spawn(move || {
                println!("Hello from a thread: {}!", i);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_start_one_thread_by_builder() {
    // 通过 Builder 可以对线程的初始状态进行更多的控制
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let handler = builder
        .spawn(move || {
            println!("Hello from a thread created by a builder!");
        })
        .unwrap();

    handler.join().unwrap();
}

#[test]
fn test_current_thread() {
    let current_thread = thread::current();

    println!(
        "current thread: {:?}, {:?}",
        current_thread.id(),
        current_thread.name()
    );

    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let handler = builder
        .spawn(move || {
            let child_thread = thread::current();
            println!(
                "child thread: {:?}, {:?}",
                child_thread.id(),
                child_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap();
}

/// park unpark 用来阻塞和唤醒线程，利用它们可以有效的利用 CPU，让暂时不满足条件的线程不可执行
#[test]
fn test_park_thread() {
    let parked_thread = thread::Builder::new()
        .spawn(move || {
            println!("Parking thread");
            thread::park();
            println!("Thread un parked");
        })
        .unwrap();

    thread::sleep(Duration::from_micros(100));

    println!("Execute Thread un parked");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}

#[test]
fn test_available_parallelism() {
    let count = thread::available_parallelism().unwrap().get();
    // 机器 的核数
    println!("available parallelism: {}", count);
}

#[test]
fn test_affinity() {
    let cores = (0..affinity::get_core_num()).step_by(2).collect::<Vec<_>>();

    println!("cores: {:?}", &cores);
}

#[test]
fn test_num_cpus() {
    let cores = num_cpus::get();
    println!("{}", cores);
}

#[test]
fn test_process_num() {
    let count = thread::available_parallelism().unwrap().get();
    println!("available parallelism: {}", count);

    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    } else {
        println!("num_threads: not supported");
    }

    let count = thread_amount::thread_amount();
    if !count.is_none() {
        println!("thread_amount: {}", count.unwrap());
    }

    let count = num_cpus::get();
    println!("num_cpus: {}", count);
}
