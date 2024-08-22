mod model;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::model::node_model::Node;
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

    #[test]
    fn test_pop() {
        let mut linked_stack = LinkedStack::new();
        linked_stack.push(1);
        linked_stack.push(2);
        linked_stack.push(3);

        let node = pop_helper(&mut linked_stack);
        // println!("Popped node: {:?}", node);
        assert_eq!(node.get_data(),3);

        let node = pop_helper(&mut linked_stack);
        // println!("Popped node: {:?}", node);
        assert_eq!(node.get_data(),2);

        let node = pop_helper(&mut linked_stack);
        // println!("Popped node: {:?}", node);
        assert_eq!(node.get_data(),1);

        // let node = pop_helper(&mut linked_stack); // THE CODE PANICS HERE
        // println!("Popped node: {:?}", node);
        // assert_eq!(node.get_data(),);

        assert_eq!(linked_stack.size(),0);

        println!("POP test passed");
    }

    #[test]
    fn test_peek() {
        let peek_element = String::from("Second element");
        let mut linked_stack = LinkedStack::new();
        linked_stack.push(String::from("First element"));
        linked_stack.push(peek_element.clone());

        if let Some(data) = linked_stack.peek() {
            assert_eq!(data.to_owned(), peek_element);
        } else {
            panic!("Stack is empty, cannot peek");
        }

        println!("PEEK test passed");
    }

    fn pop_helper<T>(stack: &mut LinkedStack<T>) -> Node<T> {
        if let Some(head) = stack.pop() { head } else {
            panic!("Stack is empty");
        }
    }
}
