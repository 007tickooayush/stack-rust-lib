mod model;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::model::stack_model::LinkedStack;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_push() {
        let mut linked_stack = LinkedStack::new();
        linked_stack.push(1);
        linked_stack.push(2);
        linked_stack.push(3);

        println!("PUSH test passed");
    }
}
