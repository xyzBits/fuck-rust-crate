fn main() {}

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use rand::{Rng, thread_rng};
    use rand::prelude::IndexedRandom;

    #[test]
    fn test_1() {
        for _ in 0..10 {
            println!("hello");
        }


        println!("{}", "world\n".repeat(10));
    }

    fn finish(name: &str) {
        println!("My job here is done. Goodbye {}", name);
    }

    fn square(x: u32) -> u32 {
        x * x
    }


    struct Point {
        x: f64,
        y: f64,
    }

    struct PointV2(f64, f64);


    #[test]
    fn test_iterate_list() {
        let items = vec![1, 2, 3, 4];
        for x in items {
            println!("{}", x);
        }
    }

    #[test]
    fn test_iterate_index_and_value_for_list() {
        let items = vec![1, 2, 3, 4];

        for (index, value) in items.iter().enumerate() {
            println!("Item {index} = {value}");
        }

        items.iter().enumerate().for_each(|(index, value)| {
            println!("Item {index} = {value}");
        })
    }

    #[test]
    fn test_create_map() {
        let mut map = BTreeMap::new();
        map.insert("one", 1);
        map.insert("two", 2);

        let map = [
            ("one", 1),
            ("two", 2)
        ].into_iter().collect::<HashMap<_, _>>();
    }


    struct BinTree<T> {
        value: T,
        left: Option<Box<BinTree<T>>>,
        right: Option<Box<BinTree<T>>>,
    }

    #[test]
    fn test_pick_random_element_from_list() {
        let list = vec![1, 2, 3, 4];
        let mut rng = rand::thread_rng();
        let choice = list.choose(&mut rng).unwrap();
        println!("{}", choice);

        let mut rng = thread_rng();

        let mut list = vec![1, 2, 3, 4];
    }

    #[test]
    fn check_contains() {
        let list = vec![1, 2, 3, 4];

        println!("{}", list.contains(&3));

        let result = list.iter().any(|item| item == &1);
        println!("result ={result}");

        let result = &list.binary_search(&4).is_ok();
        println!("result ={result}");
    }


    #[test]
    fn test_iterate_map() {
        let map = [
            ("one", 1),
            ("two", 2),
            ("three", 3)
        ].into_iter().collect::<HashMap<_, _>>();

        for (key, value) in &map {
            println!("key ={key}, value ={value}");
        }

        let result = thread_rng().gen_range(3.5f32..9.9f32);
        println!("result ={result}");
    }

    fn depth_first_traverse<T>(bt: &mut BinTree<T>, operation: fn(&mut BinTree<T>)) {
        if let Some(left) = &mut bt.left {
            depth_first_traverse(left, operation);
        }

        operation(bt);

        if let Some(right) = &mut bt.right {
            depth_first_traverse(right, operation);
        }
    }

    struct TreeNode<T> {
        value: T,
        children: Option<Vec<TreeNode<T>>>,
    }


    #[test]
    fn test_reverse_list() {
        let mut list = vec![1, 2, 3, 4, 6];
        &list.reverse();
        println!("{:?}", list);

        let result = &list.into_iter().rev().collect::<Vec<_>>();

        println!("result = {:?}", result);
    }

    fn search<T: Eq>(matrix: &Vec<Vec<T>>, target: &T) -> Option<(usize, usize)> {
        for (i, row) in matrix.iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                if *column == *target {
                    return Some((i, j));
                }
            }
        }
        None
    }

    #[test]
    fn test_swap_value() {
        let (mut a, mut b) = (99.9, -71.2);
        println!("a = {a}, b = {b}");

        (a, b) = (b, a);
        println!("a = {a}, b = {b}");


        std::mem::swap(&mut a, &mut b);

        println!("a = {a}, b = {b}");

    }


    #[test]
    fn convert_string_integer() {
        let input = "4352";

        let result = input.parse::<i32>().unwrap();
        println!("resul = {result}");

        let result = input.parse::<i32>().unwrap_or(0);
        println!("resul = {result}");

        let result = match "-099".parse::<i32>() {
            Ok(value) => value,
            Err(e) => -1
        };
        println!("resul = {result}");

    }
}


