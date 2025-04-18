pub mod data_to_sqlite;
pub mod model;
pub mod reader;
pub mod sqlite;

use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer, WriterBackend};
use std::borrow::Cow;

#[allow(clippy::derive_partial_eq_without_eq)]
// clippy 是代码检查工具，实现的 partialEq 最好也实现 eq，allow允许忽略这个告警
#[derive(Debug, Default, PartialEq, Clone)] // partialEq ==
pub struct TransferHeader<'a> {
    // 'a 告诉编译器，这个结构体中的某些字段 from_sys extend 借用的数据活多久，确保借用的数据不会在不安全的时候被销毁
    pub version: u64,
    pub modify_time: u64,
    pub from_sys: Cow<'a, str>, // Cow 类型，可以是借用字符串 &str，或者拥有字符串 String
    pub table_name_map_entries: Vec<TableNameMapEntry<'a>>,
    pub extend: Cow<'a, [u8]>, // 可以是借用的 &'a [u8] 也可以是 Vec<u8>
}

impl<'a> MessageRead<'a> for TransferHeader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> quick_protobuf::Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_uint64(bytes)?,
                Ok(16) => msg.modify_time = r.read_uint64(bytes)?,
                Ok(26) => msg.from_sys = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg
                    .table_name_map_entries
                    .push(r.read_message::<TableNameMapEntry>(bytes)?),
                Ok(42) => msg.extend = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(msg)
    }
}

impl<'a> MessageWrite for TransferHeader<'a> {
    fn write_message<W: WriterBackend>(&self, _: &mut Writer<W>) -> quick_protobuf::Result<()> {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableNameMapEntry<'a> {
    pub id: u32,
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TableNameMapEntry<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> quick_protobuf::Result<Self> {
        todo!()
    }
}

impl<'a> MessageWrite for TableNameMapEntry<'a> {
    fn write_message<W: WriterBackend>(&self, _: &mut Writer<W>) -> quick_protobuf::Result<()> {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransferItem<'a> {
    pub table_name: Cow<'a, str>,
    pub table_id: u32,
    pub key: Cow<'a, [u8]>,
    pub value: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for TransferItem<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> quick_protobuf::Result<Self> {
        todo!()
    }
}

impl<'a> MessageWrite for TransferItem<'a> {
    fn write_message<W: WriterBackend>(&self, _: &mut Writer<W>) -> quick_protobuf::Result<()> {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }
}

// cargo build 时，test 中的代码不会被编译
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::ToSql;
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
}
