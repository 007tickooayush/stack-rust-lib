use std::ops::Deref;
use crate::model::node_model::Node;

pub struct LinkedStack<T> {
    head: Option<Node<T>>,
    size: usize,
}


impl<T> LinkedStack<T> {

    pub fn new() -> Self {
        Self {
            head: None,
            size: 0
        }
    }

    pub fn push(&mut self, data: T) -> usize {
        let mut new_node = Node::new(data);

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            if let Some(head) = self.head.take() {
                new_node.next = Some(Box::new(head));
                self.head = Some(new_node);
            } else {
                self.head = Some(new_node);
            }
        }
        self.size += 1;

        self.size
    }

    pub fn pop() {
        unimplemented!("pop method not implemented yet");
    }

    pub fn peek() {
        unimplemented!("peek method not implemented yet");
    }
}