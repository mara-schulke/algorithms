// struct Node<T> {
//     value: T,
//     next: Option<Box<Node<T>>>,
// }

// impl<T> Node<T> {
//     fn new(&self, val: T) -> Self {
//         Node {
//             value: val,
//             next: None,
//         }
//     }
// }

// struct LinkedList<T> {
//     len: usize,
//     head: Option<Node<T>>,
// }

// impl<T> LinkedList<T> {
//     fn new(&self) -> Self {
//         LinkedList { len: 0, head: None }
//     }

//     fn insert(&mut self, val: T) {
//         let tmp = Node {
//             value: val,
//             next: match &self.head {
//                 Some(node) => Some(Box::new(node)),
//                 None => None,
//             },
//         };

//         // self.head = Some(tmp);
//     }
// }
