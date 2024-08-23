pub struct VectorStack<T> {
    stack : Vec<T>,
    size: usize,
}

impl<T> VectorStack<T> {
    pub fn new() -> Self {
        Self {
            stack : Vec::new(),
            size: 0
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, data: T) -> usize {
        self.stack.push(data);
        self.size += 1;
        self.size
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            println!("Stack is empty");
            return None;
        }
        self.size -= 1;
        self.stack.pop()
    }


    pub fn peek(&self) -> Option<&T>{
        if self.size == 0 {
            return None;
        }
        Some(&self.stack[self.size - 1])
    }
}