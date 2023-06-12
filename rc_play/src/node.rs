use std::rc::Rc;

// #[derive(Debug)]
// struct Node<'a, T> {
//     value: T,
//     left: Option<&'a Node<'a, T>>,
//     right: Option<&'a Node<'a, T>>,
// }

// impl<'a, T> Node<'a, T> {
//     fn new(value: T) -> Self {
//         Node {
//             value,
//             left: None,
//             right: None,
//         }
//     }

//     fn insert_left<'b: 'a>(&'b mut self, node: &'b Node<'a, T>) {
//         self.left = Some(node);
//     }

//     fn insert_right<'b: 'a>(&'b mut self, node: &'b Node<'a, T>) {
//         self.right = Some(node);
//     }
// }

// fn main() {
//     let mut root = Node::new(23);

//     let mut child_1 = Node::new(1);
//     let mut child_2 = Node::new(555);
//     root.insert_left(&child_1);
//     root.insert_right(&child_2);

//     let child_1_1 = Node::new(77);
//     child_1.insert_left(&child_1_1);

//     println!("{:?}", root);
// }



#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            value: val,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, node: Node<T>) -> Rc<Node<T>>{
        let new_node = Rc::new(node);
        self.left = Some(Rc::clone(&new_node));
        new_node
    }

    fn add_right(&mut self, node: Node<T>) -> Rc<Node<T>>{
        let new_node = Rc::new(node);
        self.right = Some(Rc::clone(&new_node));
        new_node
    }
}


fn main() {
    let mut node = Node::new(5);
    println!("{:?}", &node);
    
    let mut node_1 = Node::new(6);
    let node_23 = node.add_left(node_1);
    
    let mut node_2 = Node::new(45);
    let node_22 = node.add_right(node_2);

    println!("{:?}", &node);

    let node_3 = Node::new(888);
    node_23.add_left(node_3);

    println!("{:?}", &node_23)
}