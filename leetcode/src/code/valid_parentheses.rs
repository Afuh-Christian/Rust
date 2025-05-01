use std::{collections::HashSet, fmt::{self, Formatter}};

#[derive(Hash, PartialEq, Eq, Debug)]
enum Parentheses {
    p_open,
    p_close,
    b_open,
    b_close,
    c_open,
    c_close,
}

impl Parentheses {
    fn as_str(&self) -> &'static str {
        match self {
            Parentheses::p_open => "(", 
            Parentheses::p_close => ")",
            Parentheses::b_open => "[",
            Parentheses::b_close => "]",
            Parentheses::c_open => "{",
            Parentheses::c_close => "}",
        }
    }
}

impl fmt::Display for Parentheses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn valid_parentheses(s: String) -> bool {
    let mut stack = Vec::new();  // Stack to track opening parentheses

    // Loop through each character in the string
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => {
                // If it's an opening parenthesis, push it to the stack
                stack.push(ch);
            }
            ')' => {
                // If it's a closing parenthesis, check if the stack has a matching opening one
                if stack.last() == Some(&'(') {
                    stack.pop();  // Pop the matching opening parenthesis
                } else {
                    return false;  // If no match, the parentheses are not balanced
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => {
                // Ignore non-parentheses characters
                continue;
            }
        }
    }

    // If the stack is empty, all parentheses were properly closed
    stack.is_empty()
}

fn main() {
    let s = "{[()]}";
    let result = valid_parentheses(s.to_string());
    println!("Is the parentheses string valid? {}", result);
}
