pub trait Stack<T> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn push(&mut self, data: T) -> usize;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn flush(&mut self);
}