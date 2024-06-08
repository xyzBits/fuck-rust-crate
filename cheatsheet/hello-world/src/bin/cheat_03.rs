extern crate core;

fn main() {}

#[allow(unused)]
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use core::num;
    use std::{fs, thread, time};
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;
    use std::time::SystemTime;
    use bigdecimal::BigDecimal;
    use num_bigint::ToBigInt;

    #[test]
    fn test_reverse_string() {
        let s = "hello world";
        let reverse = s.chars().rev().collect::<String>();
        println!("reverse = {}", reverse);
    }

    #[test]
    fn test_insert_element_in_list() {
        let mut list = vec![1, 2, 3, 4];
        list.insert(2, -98);

        println!("{:?}", list);
    }

    #[test]
    fn test_thread_sleep() {
        thread::sleep(time::Duration::from_secs(5));
    }

    #[test]
    fn test_string_suffix() {
        let s = "hello world";
        let result = s.chars().take(5).collect::<String>();

        println!("result = {result}");

        let s = "lin1 \
        line 2\
        line 3";

        println!("s = {s}");

        let s = "hello world bitch";

        let chunks = s.split_whitespace().collect::<Vec<_>>();
        println!("{:?}", chunks);

        let chunks: Vec<_> = s.split(' ').collect();
        println!("{:?}", chunks);
    }


    #[test]
    fn test_map_contain_key() {
        let map = [
            (1, "one"),
            (9, "nine")
        ].into_iter().collect::<HashMap<u32, &str>>();

        println!("{}", map.contains_key(&9));

        println!("{}", map.contains_key(&34));

        println!("{}", map.values().any(|&value| (*value).eq("nine")));
    }

    #[test]
    fn test_string_list() {
        let list = vec!["hello", "world", "bitch"];
        let result = list.join(", ");
        println!("{}", result);

        let integer_list = vec![334, 53, 5];
        let sum = integer_list.iter().sum::<i32>();
        println!("sum = {sum}");

        let sum_string = sum.to_string();
        let result = format!("{sum}");
    }

    #[test]
    fn test_launch_100_task_and_wait_for_completion() {
        let handles = (0..1000)
            .map(|item| thread::spawn(move || {}))
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }
    }


    #[test]
    fn test_filter_list() {
        let list = (-23..2234).into_iter().collect::<Vec<_>>();
        let filter_list = list.iter().filter(|item| **item % 2 == 0).collect::<Vec<_>>();
    }

    #[test]
    fn test_read_file_to_string() {
        let content = fs::read_to_string("./Cargo.toml").unwrap();
        println!("{}", content.len());

        let mut file = File::open("./Cargo.toml").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        println!("{}", buffer.len());
    }

    #[test  ]
    fn write_to_standard_error() {
        eprintln!("{} is negative", -9);

        println!("{:?}", SystemTime::now());
    }

    #[test]
    fn test_big_number() {
        let a = 3.to_bigint().unwrap();

        let decimal = BigDecimal::from_str("0.8").unwrap();
        println!("{}", decimal);
    }
}