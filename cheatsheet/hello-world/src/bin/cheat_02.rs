fn main() {}

#[allow(dead_code)]
#[allow(unused)]
#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::sync::Mutex;
    use std::thread;

    #[test]
    fn test_send_value_to_thread() {
        let (sender, receiver) = channel();

        let handle = thread::spawn(move || {
            let msg = receiver.recv().unwrap();
            println!("msg = {msg}");
        });

        sender.send("hello bitch").unwrap();

        handle.join().unwrap();
    }

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_sort_by_property() {
        let mut persons = vec![
            Person { name: "hello".to_string(), age: 863 },
            Person { name: "world".to_string(), age: 99 },
            Person { name: "bitch".to_string(), age: 73 },
        ];

        persons.sort_by(|a, b| (&a.age).cmp(&b.age));

        println!("{:?}", persons);

        persons.sort_by_key(|item| item.age);
        println!("{:?}", persons);
    }

    #[test]
    fn test_execute_1000_task() {
        let handles = (0..1000).map(|i| {
            thread::spawn(move || {})
        })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }

        // (0..100).into_iter().for_each(|| {});
    }

    #[test]
    fn test_update_variable_atomically() {
        let data = 5;
        let  lock = Mutex::new(data);
        let mut guard = lock.lock().unwrap();
        *guard = 43;

        println!("{:?}", data);
    }
}