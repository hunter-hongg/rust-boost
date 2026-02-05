use std::fmt::Display;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vector<T: Clone> {
    _vec: Vec<T>,
}

impl<T: Clone> Vector<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self {
            _vec: vec,
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        self._vec
    }

    pub fn len(&self) -> usize {
        self._vec.len()
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        self._vec.get(index)
    }

    pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        self._vec.get_mut(index)
    }

    pub fn push_back(&mut self, value: T) {
        self._vec.push(value);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self._vec.pop()
    }

    pub fn push_front(&mut self, value: T) {
        self._vec.insert(0, value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self._vec.is_empty() {
            return None;
        }
        Some(self._vec.remove(0))
    }

    pub fn front(&self) -> Option<&T> {
        self._vec.get(0)
    }

    pub fn back(&self) -> Option<&T> {
        self._vec.last()
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut vec = self._vec.clone();
        vec.extend_from_slice(&other._vec);
        Self {
            _vec: vec,
        }
    }

    pub fn concat_vec(&self, other: &Vec<T>) -> Self {
        let mut vec = self._vec.clone();
        vec.extend_from_slice(other);
        Self {
            _vec: vec,
        }
    }

    pub fn append(&mut self, other: &Self) {
        let mut other = other._vec.clone();
        self._vec.append(&mut other);
    }

    pub fn append_vec(&mut self, other: &mut Vec<T>) {
        self._vec.append(other);
    }

    pub fn sort(&mut self) where T: Ord {
        self._vec.sort();
    }

    pub fn reverse(&mut self) {
        self._vec.reverse();
    }

    pub fn fliter_self(&mut self, predicate: impl Fn(&T) -> bool) {
        self._vec.retain(predicate);
    }

    pub fn fliter(&self, predicate: impl Fn(&T) -> bool) -> Vector<T> {
        Vector::new(self._vec.iter().filter(|x| predicate(x)).cloned().collect())
    }

    pub fn map_self(&mut self, mapper: impl Fn(&T) -> T) {
        self._vec.iter_mut().for_each(|x| *x = mapper(x));
    }

    pub fn map<U: Clone>(&self, mapper: impl Fn(&T) -> U) -> Vector<U> {
        Vector::new(self._vec.iter().map(|x| mapper(x)).collect())
    }

    pub fn join(&self, separator: &str) -> String where T: Display {
        let mut jstr = String::new();
        let mut first = true;
        
        for item in &self._vec {
            if !first {
                jstr.push_str(separator);
            }
            jstr.push_str(&format!("{}", item));
            first = false;
        }
        jstr
    }

    pub fn find(&self, value: &T) -> Option<usize> where T: PartialEq {
        for (i, item) in self._vec.iter().enumerate() {
            if item == value {
                return Some(i);
            }
        }
        None
    }

    pub fn find_if(&self, predicate: impl Fn(&T) -> bool) -> Option<usize> {
        for (i, item) in self._vec.iter().enumerate() {
            if predicate(item) {
                return Some(i);
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self._vec.is_empty()
    }

}