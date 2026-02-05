use std::fmt::Display;

use super::vector::Vector;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Stack<T: Clone + Ord> {
    _data: Vector<T>,
}

impl<T: Clone + Ord> Stack<T> { 
    pub fn new() -> Self {
        Self {
            _data: Vector::new(vec![]),
        }
    }

    pub fn to_vec(self) -> Vector<T> {
        self._data
    }

    pub fn new_with_vec(vec: Vec<T>) -> Self {
        Self {
            _data: Vector::new(vec),
        }
    }

    pub fn new_with_vector(vector: Vector<T>) -> Self {
        Self {
            _data: vector,
        }
    }

    pub fn push(&mut self, value: T) {
        self._data.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self._data.pop_back()
    }

    pub fn top(&self) -> Option<&T> {
        self._data.back()
    }

    pub fn size(&self) -> usize {
        self._data.len()
    }

    pub fn is_empty(&self) -> bool {
        self._data.is_empty()
    }

    pub fn join(&self, separator: &str) -> String where T: Display {
        self._data.join(separator)
    }

}