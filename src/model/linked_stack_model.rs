use std::sync::Arc;
use crate::core::core_stack_model::{Stack, StackError, StackResult};

pub struct LinkedStack<T> {
    head: Option<Node<T>>,
    size: usize,
}


impl<T> Stack<T> for LinkedStack<T>
where T: Clone {

    fn new() -> Self {
        Self {
            head: None,
            size: 0
        }
    }

    fn size(&self) -> usize {
        self.size
    }
    fn push(&mut self, data: T) -> usize {
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

    fn pop(&mut self) -> StackResult<T> {
        if let Some(head) = self.head.take() {
            self.head = match head.next {
                Some(next) => {
                    match Arc::try_unwrap(next) {
                        Ok(next_node) => Some(next_node),
                        Err(_) => return Err(StackError::StackError("Failed to unwrap Arc, Can not perform POP operation"))
                    }
                },
                None => None
            };
            self.size -= 1;
            Ok(head.data)
        } else {
            Err(StackError::StackEmpty)
        }
    }

    fn peek(&mut self) -> StackResult<&T>{
        if let Some(head) = &self.head {
            Ok(head.peek())
        } else {
            Err(StackError::StackEmpty)
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn flush(&mut self) {
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