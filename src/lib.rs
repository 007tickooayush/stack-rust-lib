pub mod model {
    pub mod linked_stack_model;
    pub mod vector_stack_model;
}

pub mod core {
    pub mod core_stack_model;
    pub mod error;
}
#[cfg(test)]
mod tests {
    use crate::core::core_stack_model::Stack;
    use crate::model::linked_stack_model::LinkedStack;
    use crate::model::vector_stack_model::VectorStack;

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

        let popped_data = pop_helper(&mut linked_stack);
        // println!("Popped node: {:?}", node);
        assert_eq!(popped_data, 3);
        assert_eq!(linked_stack.size(),2);

        let popped_data = pop_helper(&mut linked_stack);
        println!("Popped node: {:?}", popped_data);
        assert_eq!(popped_data,2);

        let popped_data = pop_helper(&mut linked_stack);
        println!("Popped node: {:?}", popped_data);
        assert_eq!(popped_data,1);

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

        let peeked_data = linked_stack.peek().unwrap().to_owned();
        assert_eq!(peeked_data, peek_element);
        // if let Some(data) = linked_stack.peek() {
        //     assert_eq!(data.to_owned(), peek_element);
        // } else {
        //     panic!("Stack is empty, cannot peek");
        // }

        println!("PEEK test passed");
    }

    #[test]
    fn test_flush() {
        let mut linked_stack = LinkedStack::new();
        linked_stack.push(1);
        linked_stack.push(2);
        linked_stack.push(3);

        linked_stack.flush();

        assert_eq!(linked_stack.size(),0);
        assert_eq!(linked_stack.is_empty(), true);

        println!("FLUSH test passed");
    }

    fn pop_helper<T>(stack: &mut LinkedStack<T>) -> T
    where T: Clone {
        stack.pop().unwrap()
    }

    #[test]
    fn test_push_vector() {
        let mut vector_stack = VectorStack::new();

        vector_stack.push(1);
        vector_stack.push(2);
        vector_stack.push(3);

        assert_eq!(vector_stack.size(), 3);
        println!("PUSH test passed");
    }


    #[test]
    fn test_pop_vector() {
        let mut vector_stack = VectorStack::new();

        vector_stack.push(1);
        vector_stack.push(2);

        let popped_data = vector_stack.pop().unwrap();
        assert_eq!(popped_data,2);
        // if let Some(el) = vector_stack.pop() {
        //     assert_eq!(el,2);
        // } else {
        //     panic!("Stack is empty");
        // }

        assert_eq!(vector_stack.size(), 1);

        println!("POP test passed");
    }

    #[test]
    fn test_peek_vector() {
        let mut vector_stack = VectorStack::new();
        let el = String::from("Some data String");
        vector_stack.push(String::from("Some String data"));
        vector_stack.push(el.clone());

        let peeked_data = vector_stack.peek().unwrap().to_owned();
        assert_eq!(peeked_data, el);
    }

    #[test]
    fn test_flush_vector() {
        let mut vector_stack = VectorStack::new();
        vector_stack.push(1);
        vector_stack.push(2);
        vector_stack.push(3);
        vector_stack.push(4);

        vector_stack.flush();

        assert_eq!(vector_stack.size(), 0);
    }

    #[test]
    #[should_panic]
    fn test_pop_vector_panic() {
        let mut vector_stack = VectorStack::new();
        vector_stack.push("Some Data");
        vector_stack.pop().unwrap();

        match vector_stack.pop() {
            Ok(_) => {},
            Err(e) => {
                println!("ERROR OCCURRED: {}", e);
                panic!("Panic test passed");
            }
        }
    }

}
