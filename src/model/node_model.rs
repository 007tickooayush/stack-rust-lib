#[derive(Debug)]
pub struct Node<T> {
    data: T,
    pub next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Self {
            data,
            next: None,
        }
    }

    pub fn get_data(self) -> T {
        self.data
    }
}