use std::pin::Pin;

#[test]
fn test_01() {
    let mut value = 42; // 一个普通的数字
    let pinned_value = Pin::new(&mut value);
    *pinned_value.get_mut() += 100;

    println!("{}", value);
}

#[test]
fn test_02() {
    let mut value = "hello".to_string(); // 一个普通的数字
    let pinned_value = Pin::new(&mut value);
    *pinned_value.get_mut() = "world".to_string();

    println!("{}", value);

    // let data = pinned_value;
}

struct SelfReferential {
    data: String,
    self_ref: *const String,
}

impl SelfReferential {
    fn new(data: String) -> Self {
        let mut s = SelfReferential {
            data,
            self_ref: std::ptr::null(),
        };

        // self_ref 指向 data
        s.self_ref = &s.data as *const String;
        s
    }
}

#[test]
fn test_03() {
    let s1 = SelfReferential::new("hello".to_string());
    let s2 = s1; // 此时，s2.self_ref 仍然指向 s1.data 的旧地址，访问会出错

    // drop(s1);
    println!("{:p}", s2.self_ref);
}

struct SelfReference<'a> {
    a: String,
    b: &'a String,
}

#[test]
fn test_04() {
    let s = "hello world".to_string();

    // 因为 s 已经 move ，不能再借用
    // let _ = SelfReference {
    //     a: s,
    //     b: &s,
    // };
}

#[derive(Debug)]
struct SelfReferenceV2 {
    a: String,
    b: *const String,
}

impl SelfReferenceV2 {
    fn new(msg: &str) -> Self {
        Self {
            a: msg.to_string(),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let ptr_to_a = &self.a as *const String;
        self.b = ptr_to_a;
    }

    fn get_a(&self) -> &str {
        &self.a
    }

    fn get_b(&self) -> &str {
        unsafe { &*self.b }
    }
}

#[test]
fn test_05() {
    let mut sr_1 = SelfReferenceV2::new("hello");
    sr_1.init();

    let mut sr_2 = SelfReferenceV2::new("world");
    sr_2.init();

    println!("sr_1 a: {}, b: {}", sr_1.get_a(), sr_1.get_b());
    println!("sr_2 a: {}, b: {}", sr_2.get_a(), sr_2.get_b());
}

#[test]
fn test_06() {
    let mut sr_1 = SelfReferenceV2::new("hello");
    sr_1.init();

    let mut sr_2 = SelfReferenceV2::new("world");
    sr_2.init();

    println!("before swap");
    println!("sr_1 a: {}, b: {}", sr_1.get_a(), sr_1.get_b());
    println!("sr_2 a: {}, b: {}", sr_2.get_a(), sr_2.get_b());

    std::mem::swap(&mut sr_1, &mut sr_2);
    println!("\n after swap");
    println!("sr_1 a: {}, b: {}", sr_1.get_a(), sr_1.get_b());
    println!("sr_2 a: {}, b: {}", sr_2.get_a(), sr_2.get_b());

}
