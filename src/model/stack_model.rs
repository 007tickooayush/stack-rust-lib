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

    pub fn size(&self) -> usize {
        self.size
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

    pub fn pop(&mut self) -> Option<Node<T>> {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take().map(|node| *node);
            self.size -= 1;

            Some(head)
        } else {
            println!("Stack is empty");
            None
        }
        // unimplemented!()
    }

    pub fn peek(&self) -> Option<&T>{
        if let Some(head) = &self.head {
            Some(head.peek())
        } else {
            None
        }
    }
}