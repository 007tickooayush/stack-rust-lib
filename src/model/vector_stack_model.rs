use crate::core::core_stack_model::Stack;

pub struct VectorStack<T> {
    stack : Vec<T>,
    size: usize,
}

impl<T> Stack<T> for VectorStack<T> {
    fn new() -> Self {
        Self {
            stack : Vec::new(),
            size: 0
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

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            println!("Stack is empty");
            return None;
        }
        self.size -= 1;
        self.stack.pop()
    }


    fn peek(&self) -> Option<&T>{
        if self.size == 0 {
            return None;
        }
        Some(&self.stack[self.size - 1])
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn flush(&mut self) {
        self.stack.clear();
        self.size = 0;
    }
}