use crate::core::Stack;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Operand(i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
}

#[allow(dead_code)]
fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Add | Operator::Subtract => 2,
        Operator::Multiply => 3,
    }
}

pub fn infix_to_postfix(input: &str) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in input.split_whitespace() {
        match token {
            "+" => {
                while let Some(top_op) = operator_stack.last() {
                    if *top_op == Operator::Multiply {
                        break;
                    }
                    output.push(Token::Operator(operator_stack.pop().unwrap()));
                }
                operator_stack.push(Operator::Add);
            }
            "-" => {
                while let Some(top_op) = operator_stack.last() {
                    if *top_op == Operator::Multiply {
                        break;
                    }
                    output.push(Token::Operator(operator_stack.pop().unwrap()));
                }
                operator_stack.push(Operator::Subtract);
            }
            "*" => {
                operator_stack.push(Operator::Multiply);
            }
            "(" => {
                operator_stack.push(Operator::Multiply);
            }
            ")" => {
                while let Some(top_op) = operator_stack.last() {
                    if *top_op == Operator::Multiply {
                        operator_stack.pop();
                        break;
                    }
                    output.push(Token::Operator(operator_stack.pop().unwrap()));
                }
            }
            _ => {
                let operand = token.parse().unwrap();
                output.push(Token::Operand(operand));
            }
        }
    }

    while let Some(top_op) = operator_stack.pop() {
        output.push(Token::Operator(top_op));
    }

    output
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Operator(op) => write!(f, "{}", op),
            Token::Operand(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
        }
    }
}

pub fn evaluate_postfix(tokens: &[Token]) -> Option<i32> {
    let mut stack = Stack::new();

    for token in tokens {
        match token {
            Token::Operand(num) => {
                stack.push(*num);
            }
            Token::Operator(operator) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                let result = match operator {
                    Operator::Add => a + b,
                    Operator::Subtract => a - b,
                    Operator::Multiply => a * b,
                };
                stack.push(result);
            }
        }
    }

    stack.pop()
}