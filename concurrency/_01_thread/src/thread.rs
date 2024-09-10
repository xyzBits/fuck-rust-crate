use std::cell::RefCell;
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

#[test]
fn test_start_thread_with_sleep() {
    // 保证当前线程 sleep 指定的时间，它为它会阻塞当前线程，所以不要在异步的代码中调用它，
    // 如果时间设置为 0 ， unix 类平台会立即 返回，
    // 如果只想让度出时间片，不用设置为 0，而是调用 yield_now 函数
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2_000));
        println!("Hello from thread1!");
    });

    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1_000));
        println!("Hello from thread2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn test_start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        // 暂时的让出同 cpu 的 time slice ，后面会立即 被调度
        thread::yield_now();
        println!("yield from thread1");
    });

    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield from thread2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn test_thread_park_2() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2_000));

        thread::park();

        println!("Hello from a park thread in case of unpark first");
    });

    handle.thread().unpark();

    handle.join().unwrap();
}

#[test]
fn test_wrong_start_threads_without_scoped() {
    // let mut a = vec![1, 2, 3];
    // let mut x = 0;
    //
    // thread::spawn(move || {
    //    println!("Hello from the first scoped thread");
    //     dbg!(&a);
    // });
    //
    // thread::spawn(move || {
    //    println!("Hello from the second scoped thread");
    //     dbg!(&a);
    // });
    //
    // println!("Hello from the main thread");
    //
    // a.push(4);
    //
    // assert_eq!(a.len(), x);
}

#[test]
fn test_wrong_start_scoped_thread() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            dbg!(&a);
        });

        s.spawn(|| {
            println!("hello from the second scoped thread");
            x += a[0] + a[2];
        });

        println!("hello from the main thread");
    });

    a.push(4);

    assert_eq!(x, a.len());
}

#[test]
fn test_start_thread_with_thread_local() {
    // 在这里，定义了 一个 thread local 的 key ，在外部线程和两个子线程中都修改了，但是修改 counter 只会影响本线程，
    // 子线程修改后，不会影响 main 线程
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));

    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    let handle1 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 3;
        });

        COUNTER.with(|c| {
            println!("Hello from thread7, c = {}", *c.borrow());
        });
    });

    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 4;
        });

        COUNTER.with(|c| {
            println!("Hello from thread8, c = {}", *c.borrow());
        });
    });

    COUNTER.with(|c| {
        *c.borrow_mut() += 9;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|c| {
        println!("Hello from main, c = {}", *c.borrow());
    });
}

#[test]
fn test_start_one_thread_with_move() {
    let x = 100;

    // 当我们在线程中引用变量 x 时，我们使用了 move，当我们没有引用变量时，没有使用 move
    // 因为 x 是 i32 类型的，实现了 copy trait，实际在 move 的时候，复现的是它的值
    //

    let handle = thread::spawn(move || {
        println!("hello from a thread with move, x={}", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(move || {
        println!("hello from a thread with move again, x={}", x);
    });
    handle.join().unwrap();

    let handle = thread::spawn(|| {
        println!("hello from a thread with move without move");
    });

    handle.join().unwrap();
}

// 如果 把 x 替换成一个 未实现 copy 的类型，类似的代码就无法通过编译了，因为所有权已经转移给第一个子线程了
#[test]
fn test_start_one_thread_with_move_2() {
    let x = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("hello from a thread with move, x={:?}", x);
    });

    handle.join().unwrap();

    // let handle = thread::spawn(move || {
    //     println!("hello from a thread with move again, x={}", x);
    // });
    // handle.join().unwrap();

    let handle = thread::spawn(|| {
        println!("hello from a thread with move without move");
    });

    handle.join().unwrap();
}


#[test]
fn test_control_thread() {
}

#[test]
fn test_start_thread_with_priority() {
}

#[test]
fn test_panic_thread() {
    println!("Hello, world");

    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2_000));
        panic!("panic thread");
    });

    match handle.join() {
        Ok(res ) => {
        println!("All is well, res = {:?}", res);
    },
        Err(e) => {
            println!("Got an error = {:?}", e);
        }
    }


}