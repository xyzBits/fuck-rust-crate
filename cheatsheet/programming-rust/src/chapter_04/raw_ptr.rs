#[test]
fn test_01() {
    let x = 42;
    let raw_ptr: *const i32 = &x;
    let raw_ptr = &x as *const i32;

    // 必须使用 unsafe 块解引用
    unsafe {
        let value = *raw_ptr;
        println!("value = {}", value);
    }
}