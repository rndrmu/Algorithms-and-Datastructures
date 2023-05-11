
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;



pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }

    pub fn empty(&self) -> bool {
        self.items.is_empty()
    }
}
