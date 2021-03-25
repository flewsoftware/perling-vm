use crate::register;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub struct STACK {
    pub content: Vec<i32>,
}

impl STACK {
    pub fn add_register(&mut self, register: register::REGISTER) {
        self.content.push(register.content);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_register_method() {
        let test_register = register::REGISTER {
            content: 2,
            locked: false,
        };
        let mut test_stack = STACK {
            content: vec![3, 5],
        };
        test_stack.add_register(test_register);
        assert_eq!(
            test_stack,
            STACK {
                content: vec![3, 5, 2],
            },
        );
    }
}
