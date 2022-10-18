use crate::node::{self, Node};
struct Heap<T> {
    vrsta: Vec<node::Node<T>>,
}
impl<T> Heap<T> {
    fn new() -> Heap<T> {
        Heap { vrsta: vec![] }
    }
    /// inserts a value into the heap
    pub fn insert(&mut self, value: T) -> bool {
        todo!()
    }
}
#[cfg(test)]
mod heap_tests {
    use crate::node::Node;

    use super::Heap;

    #[test]
    fn delete_min() {
        let mut heap1 = Heap::<i32>::new();
        assert!(heap1.insert(1));
        assert!(heap1.insert(2));
        assert!(heap1.insert(4));
        assert!(heap1.insert(5));
        assert!(heap1.insert(3));
    }
    #[test]
    fn get_min() {
        assert!(true);
    }
    #[test]
    fn insert() {
        assert!(true);
    }
    #[test]
    fn resize_array() {
        assert!(true);
    }
    #[test]
    fn merge() {
        assert!(true);
    }
}
