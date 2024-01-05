use std::{collections::VecDeque, cell::RefCell};

trait Mocki<T> {
    fn add_value(&self, val: T);

    fn mock_once(&self) -> T;

    fn register_call(&self, times: u32);

    fn get_calls(&self) -> u32;

    fn value_count(&self) -> usize;
}

struct Mock<T> {
    values: RefCell<VecDeque<T>>,
    calls: RefCell<u32>, 
}

impl<T> Mock<T> {
    #[allow(unused)]
    fn new() -> Mock<T> {
        Mock { values: RefCell::new(VecDeque::new()), calls: RefCell::new(0) }
    }
}

impl<T> Mocki<T> for Mock<T> {
    fn add_value(&self, val: T) {
        self.values.borrow_mut().push_back(val);
    }

    fn mock_once(&self) -> T {
        self.register_call(1);
        self.values.borrow_mut().pop_front().unwrap()
    }

    fn register_call(&self, times: u32) {
        *self.calls.borrow_mut() += times;
    }

    fn get_calls(&self) -> u32 {
        *self.calls.borrow()
    }

    fn value_count(&self) -> usize {
        (*self.values.borrow()).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mock = Mock::new();
        let value_to_mock = "one";
        mock.add_value(value_to_mock.to_owned());
        mock.add_value("two".to_string());
        let mocked_value = mock.mock_once();
        assert_eq!(value_to_mock, mocked_value.as_str());
        assert!(mock.get_calls() == 1);
    }
}
