#[derive(Debug)]
pub enum List<T> {
    Empty,
    // recursive type List has infinite size
    // recursive without indirection
    // Elem(i32, List),

    Elem(T, Box<List<T>>),
}

#[cfg(test)]
mod list_tests {
    use crate::zero::List;

    #[test]
    fn test_enum_list() {
        // first element is located in stack
        // last element is also needed to allocate space
        let list = List::Elem(1,
                              Box::new(List::Elem(2,
                                                  Box::new(List::Elem(3,
                                                                      Box::new(List::Empty))))));

        println!("{:?}", list);
    }
}
