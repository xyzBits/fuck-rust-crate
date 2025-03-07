macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

#[test]
fn test_01() {
    say_hello!();
}

macro_rules! repeat_print {
    ($msg:expr, $times:expr) => {
        for _ in 0..$times {
            println!("{} ", $msg);
        }
    };
}

#[test]
fn test_02() {
    repeat_print!("Rust is awesome!", 3);
}

macro_rules! repeat_print_v2 {
    ($msg:expr, $times:expr) => {
        if let Ok(n) = stringify!($msg).parse::<usize>() {
            for _ in 0..n {
                println!("{} ", $msg);
            }
        } else {
            eprintln!("Error: $times must be a positive number");
        }
    };
}

#[test]
fn test_03() {
    repeat_print_v2!("Hello, world!", "3");

    vec![1, 2, 3];
}

macro_rules! repeat_print_v3 {
    ($msg:expr, $times:literal) => {{
        let times_str = stringify!($times);
        println!("times_str = {} ", times_str);
        match times_str.parse::<usize>() {
            Ok(n) => {
                for _ in 0..n {
                    println!("{}", $msg);
                }
            }
            Err(_) => {
                panic!("Invalid number: {}", times_str);
            }
        }
    }};
    ($msg:expr, $times:expr) => {
        for _ in 0..$times {
            println!("{}", $msg);
        }
    };
}

#[test]
fn test_04() {
    repeat_print_v3!("Hello, world!", 3);
    repeat_print_v3!("Hello, world!", "3");
    repeat_print_v3!("Hello, world!", 7 + 8);

    let res = "3".parse::<usize>().unwrap();
    println!("{}", res);
}

macro_rules! my_vec {
    () => {{
        Vec::new()
    }};


    ($($element: expr),+ $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($element);)+
        temp_vec
    }}
}

#[test]
fn test_05() {
    let v = my_vec![1, 2, 3, 4];
}
