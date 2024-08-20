pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        unimplemented!("new method not implemented yet");
    }
}