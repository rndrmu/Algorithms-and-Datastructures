#![allow(dead_code, unused_variables, unused_mut, unused_imports, unused_macros, non_snake_case)]


use crate::calcs::{infix_to_postfix, evaluate_postfix};

mod calcs;
mod core;

fn main() {
    let input = "( 5 + 3 * 4 - 2 ) * 3";
    let postfix = infix_to_postfix(input);
    let tokens_stringified: String = postfix.iter().map(|t| format!("{} ", t)).collect();
    println!("{} => {}", input, tokens_stringified);
    
    let eval_result = evaluate_postfix(&postfix);
    println!("{eval_result:?}");

}
