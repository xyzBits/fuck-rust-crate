pub mod transfer;

// cargo build 时，test 中的代码不会被编译
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    // Cow 是什么
    // Cow Clone on Write 是一个设计，如果你去图书馆
    // 只需要看书，不带走，可以用借书，Cow::Borrowed
    // 要把书带回家，或者在书上写字，就要拥有一本书 Cow::Owned
    // from_sys: Cow<'a, str> 可能是从字节流中借来的字符串 &'a str，也可能是新创建的字符串 String
    // extend: Cow<'a, [u8]> 可能是借来的字符串 &'a [u8] ，也可能是新创建的 Vec<u8>
    // 为什么用 Cow
    // 节省内存，如果数据可以直接借用字节流中的内容，就不用复制一份新的
    // 灵活性：如果后续需要修改数据，可以把 Borrowed 变为 Owned，通过 clone 数据
    // 'a 告诉编译器，我这里的结构休整的 from_sys extend 借用了某些数据，这些数据至少得活到 'a 这么久，不然我没法用，

    #[derive(Debug)]
    struct User<'a> {
        name: Cow<'a, str>,
        addr: Cow<'a, str>,
        age: u32,
    }

    impl<'a> User<'a> {
        pub fn new(name: impl Into<Cow<'a, str>>, addr: impl Into<Cow<'a, str>>, age: u32) -> Self {
            User {
                name: name.into(),
                addr: addr.into(),
                age,
            }
        }

        pub fn set_name(&mut self, new_name: impl Into<Cow<'a, str>>) {
            self.name = new_name.into();
        }

        pub fn set_addr(&mut self, new_addr: impl Into<Cow<'a, str>>) {
            self.addr = new_addr.into();
        }

        pub fn update_name(&mut self) {
            self.name.to_mut().push_str("!!!");
        }
    }

    #[cfg(not)]
    fn some_function(user: &mut User) {
        let temp_str = String::from("Hello"); // temp_str 在函数结束时销毁
        // borrowed value does not live long enough
        let temp_ref = temp_str.as_str(); // temp_ref 与 temp_str 的生命周期相同
        let cow = Cow::Borrowed(temp_ref); // 借用 temp_ref
        user.set_name(cow); // 尝试将 cow 传入 set_name
    }

    #[test]
    fn test_01() {
        let mut user = User::new("Alice".to_owned(), "USA".to_string(), 23);

        user.set_name("Bob");
        println!("user: {:?}", user);
        user.update_name();

        println!("user: {:?}", user);

        // some_function(&mut user);
        println!("user: {:?}", user);
    }

    // 'a 告诉编译器，结构体内部包含引用，并且这个引用的生命周期和 'a 有关
    struct Book<'a> {
        // 'a 是一个生命周期参数，表示 author 引用的生命周期
        author: &'a str, // 'a 确保 author 指向的字符串数据至少在 Book 实例存在期间是有效的
        title: String,
    }

    #[cfg(not)]
    fn create_book() -> Book<'static> {
        let author_name = String::from("Bob");
        Book {
            author,
            title: String::from("Alice"),
        }
    }
}
