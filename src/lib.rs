//! Simple mock realization
//! 
//! it will be more likely refactored and extended, I hope)
//! 

use std::{cell::RefCell, collections::VecDeque};

pub trait Mocki<T> {
    /// Adds a value to mock
    fn add_value(&self, val: T) -> &Self;

    /// Returns a mocked value
    /// and counts mocked function call
    fn mock_once(&self) -> T;

    /// Counts a call of mocked function
    fn register_call(&self, times: u32) -> &Self;

    /// Returns a calls count of mocked function
    fn calls(&self) -> u32;

    /// Returns a count of mocked values
    fn value_count(&self) -> usize;
}

pub struct Mock<T> {
    values: RefCell<VecDeque<T>>,
    calls: RefCell<u32>,
}

impl<T> Mock<T> {
    pub fn new() -> Mock<T> {
        Mock {
            values: RefCell::new(VecDeque::new()),
            calls: RefCell::new(0),
        }
    }
}

impl<T> Mocki<T> for Mock<T> {
    fn add_value(&self, val: T) -> &Self {
        self.values.borrow_mut().push_back(val);
        self
    }

    fn mock_once(&self) -> T {
        self.register_call(1);
        self.values.borrow_mut().pop_front().unwrap()
    }

    fn register_call(&self, times: u32) -> &Self {
        *self.calls.borrow_mut() += times;
        self
    }

    fn calls(&self) -> u32 {
        *self.calls.borrow()
    }

    fn value_count(&self) -> usize {
        (*self.values.borrow()).len()
    }
}

impl Default for Mock<String> {
    fn default() -> Self {
        Mock::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mock = Mock::new();
        let value_to_mock = "one";
        mock.add_value(value_to_mock.to_owned());
        mock.add_value("two".to_string());
        let mocked_value = mock.mock_once();
        assert_eq!(value_to_mock, mocked_value.as_str());
        mock.mock_once();
        assert!(mock.calls() == 2);
        assert_eq!(mock.value_count(), 0);
    }
}
