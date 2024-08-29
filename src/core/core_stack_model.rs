pub type StackResult<T> = Result<T,StackError>;
pub trait Stack<T>
where T: Clone {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn push(&mut self, data: T) -> usize;
    fn pop(&mut self) -> StackResult<T>;
    fn peek(&mut self) -> StackResult<&T>;
    fn is_empty(&self) -> bool;
    fn flush(&mut self);
}

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum StackError {
    StackOverflow,
    StackUnderflow,
    StackEmpty,
    StackError(&'static str),
}

impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::StackError(e) => {
                write!(f, "Stack Error: {:?}", e)
            },
            _ => write!(f, "Stack Error: {:?}", self)
        }
    }
}

impl Error for StackError {
    fn description(&self) -> &str {

        match self {
            StackError::StackOverflow => "Stack Overflow",
            StackError::StackUnderflow => "Stack Underflow",
            StackError::StackEmpty => "Stack is empty",
            StackError::StackError(e) => e,
        }
    }
}