
#[derive(Debug)]
pub struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Node>),
}

#[cfg(test)]
#[allow(unused)]
#[allow(dead_code)]
mod first_tests {
    use crate::first::{Link, Node};

    #[test]
    fn test_1() {
        let node2 = Node { elem: 2, next: Link::Empty };
        let node1 = Node { elem: 1, next: Link::More(Box::new(node2)) };

        let list = Box::new(node1);

        println!("{:?}", list);


    }
}