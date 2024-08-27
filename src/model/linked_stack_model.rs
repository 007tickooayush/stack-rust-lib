use std::sync::Arc;

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
                new_node.next = Some(Arc::new(head));
                self.head = Some(new_node);
            } else {
                self.head = Some(new_node);
            }
        }
        self.size += 1;

        self.size
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            // self.head = head.next.take()
            self.head = match head.next {
                Some(head) => {
                    Some(Arc::try_unwrap(head).unwrap_or_else(|_| panic!("Can not perform the pop operation")))
                },
                None => None
            };
            self.size -= 1;

            Some(head.data)
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

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn flush(&mut self) {
        self.head = None;
        self.size = 0;
    }
}


#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Arc<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Self {
            data,
            next: None,
        }
    }

    pub fn peek(&self) -> &T {
        &self.data
    }
}