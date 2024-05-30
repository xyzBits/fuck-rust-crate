use std::fmt::Debug;
use std::ops::Bound::Included;
use std::sync::Mutex;

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().unwrap();

    *lock += 1;


    do_something_async().await;
}

async fn do_something_async() {}

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(1);

    // tokio::spawn(increment_and_do_stuff(&mutex));
    increment_and_do_stuff(&mutex).await;
    println!("{:?}", mutex);

}