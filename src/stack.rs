use crate::register;
use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct STACK {
    pub content: Vec<i32>,
}

impl Add<register::REGISTER> for STACK {
    type Output = STACK;

    fn add(self, other: register::REGISTER) -> STACK {
        let mut temp_content = self.content;
        temp_content.push(other.content);
        return STACK {
            content: temp_content,
        };
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
        let test_stack = STACK {
            content: vec![3, 5],
        };
        assert_eq!(
            test_stack + test_register + test_register,
            STACK {
                content: vec![3, 5, 2, 2],
            },
        );
    }

}
