use std::fmt;
use std::rc::Rc;

use crate::collections::linked_list::LinkedList;
struct Deque<T: fmt::Display> {
    elements: Option<LinkedList<Rc<T>>>,
    first: Option<Rc<T>>,
    last: Option<Rc<T>>,
}

impl<T: fmt::Display> Deque<T> {
    pub fn empty() -> Self {
        Deque::<T> {
            elements: Some(LinkedList::<Rc<T>>::empty()),
            first: None,
            last: None,
        }
    }

    pub fn append(&mut self, element: T) -> Result<Rc<T>, &'static str> {
        let elem_rc = Rc::new(element);
        match self.elements.take() {
            Some(elements) => {
                let new_elements = elements.append(Rc::clone(&elem_rc));
                let new_last = Rc::clone(&elem_rc);
                self.elements = Some(new_elements);
                self.first = Some(self.first.take().unwrap_or(Rc::clone(&new_last)));
                self.last = Some(Rc::clone(&new_last));
                Ok(Rc::clone(&new_last))
            },
            None => Err("Illegal Deque struct state. Please use Deque constructors.")
        }
    }
}

#[cfg(test)]
mod test_deque {
    use super::*;

    #[test]
    fn test_empty() {
        let actual = Deque::<u8>::empty();
        assert_eq!(actual.elements, Some(LinkedList::<Rc<u8>>::empty()));
        assert_eq!(actual.first, None);
        assert_eq!(actual.last, None);
    }

    #[test]
    fn test_append_from_empty_deque() {
        let mut deque = Deque::<u8>::empty();
        let result = deque.append(9);
        
        assert_eq!(result, Ok(Rc::new(9)));

        let expected_ll = LinkedList::<Rc<u8>>::empty().append(Rc::new(9));
        let expected_elements = Some(expected_ll);
        assert_eq!(deque.elements, expected_elements);
        assert_eq!(deque.first, Some(Rc::new(9)));
        assert_eq!(deque.last, Some(Rc::new(9)));
    }

    #[test]
    fn test_append_multiple() {
        let mut deque = Deque::<u8>::empty();
        let _ = deque.append(9);
        let result = deque.append(42);

        assert_eq!(result, Ok(Rc::new(42)));

        let expected_ll = LinkedList::<Rc<u8>>::empty().append(Rc::new(9)).append(Rc::new(42));
        let expected_elements = Some(expected_ll);
        assert_eq!(deque.elements, expected_elements);
        assert_eq!(deque.first, Some(Rc::new(9)));
        assert_eq!(deque.last, Some(Rc::new(42)));
    }

    #[test]
    fn test_append_illegal_state() {
        let mut deque = Deque { elements: None, first: None, last: None };
        assert_eq!(deque.append(9), Err("Illegal Deque struct state. Please use Deque constructors."))
    }
}
