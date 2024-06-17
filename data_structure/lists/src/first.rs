use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;

        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }

            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }

        result
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         self.head.drop();
//     }
// }
//
// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {}
//             Link::More(ref mut node) => {
//                 node.drop();
//             }
//         }
//
//     }
// }
//
// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         dealloc(self.ptr)
//     }
// }
//
// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }


// impl Drop for List {
//     fn drop(&mut self) {
//         let mut link = mem::replace(&mut self.head, Link::Empty);
//         while let Link::More (mut node) = link {
//
//
//         }
//     }
// }

#[cfg(test)]
#[allow(unused)]
#[allow(dead_code)]
mod first_tests {
    use std::mem;

    use crate::first::{Link, List, Node};

    #[test]
    fn test_1() {
        let node2 = Node { elem: 2, next: Link::Empty };
        let node1 = Node { elem: 1, next: Link::More(Box::new(node2)) };

        let list = List { head: Link::More(Box::new(node1)) };

        println!("{:?}", list);
    }

    #[test]
    fn test_mem_replace() {
        let mut v = vec![1, 2, 3];

        // move src into the referenced dest, returning the previous dest value
        let old_v = mem::replace(&mut v, vec![4, 5, 6]);
        println!("v = {:?}", v);
        println!("old_v = {:?}", old_v);
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100_000 {
            list.push(i);
        }

        drop(list);
    }
}