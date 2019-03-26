struct Node<T> {
    value: T,
    next: Box<Node<T>>,
}

struct LinkedList<T> {
    size: usize,
    head: Node<T>,
}

impl<T> LinkedList<T> {
    fn insert(&self, val: T) {
        self.head = Node {
            value: val,
            next: Box::new(self.head),
        }
    }
}
