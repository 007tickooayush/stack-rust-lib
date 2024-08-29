use crate::core::core_stack_model::{Stack, StackError, StackResult};

#[derive(Clone)]
pub struct VectorStack<T> {
    stack: Vec<T>,
    size: usize,
}

impl<T> Stack<T> for VectorStack<T>
where T: Clone {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn push(&mut self, data: T) -> usize {
        self.stack.push(data);
        self.size += 1;
        self.size
    }

    fn pop(&mut self) -> StackResult<T> {
        if self.size == 0 {
            Err(StackError::StackUnderflow)
        } else {
            self.size -= 1;
            if let Some(top) = self.stack.pop() {
                Ok(top)
            } else {
                Err(StackError::StackEmpty)
            }
        }
    }


    fn peek(&mut self) -> StackResult<&T> {
        if self.size == 0 {
            Err(StackError::StackEmpty)
        } else {
            Ok(self.stack.last().unwrap())
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn flush(&mut self) {
        self.stack.clear();
        self.size = 0;
    }
}