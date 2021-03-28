use std::{fmt, rc::Rc};
use std::string::ToString;

#[derive(PartialEq)]
enum LinkedList<T: fmt::Display> {
    Nil,
    Tail { head: T },
    Link { head: T, tail: Box<LinkedList<T>> },
}

impl<T: fmt::Display> LinkedList<T> {
    pub fn empty() -> Self {
        LinkedList::Nil
    }

    pub fn new_head(element: T) -> Self {
        LinkedList::Tail { head: element }
    }

    pub fn append(self, element: T) -> Self {
        match self {
            LinkedList::Nil => LinkedList::new_head(element),
            LinkedList::Tail { head } => {
                let new_tail = Box::new(LinkedList::new_head(element));
                LinkedList::Link { head, tail: new_tail }
            },
            LinkedList::Link { head, tail } => {
                let new_tail = Box::new(tail.append(element));
                LinkedList::Link { head, tail: new_tail }
            }
        }
    }

   fn to_string(&self) -> String {
       format!("List({})", self.str_loop("".to_string()))
   }

   fn str_loop(&self, acc: String) -> String {
       match self {
           LinkedList::Nil => acc,
           LinkedList::Tail { head } => format!("{}, {})", acc, head),
           LinkedList::Link { head, tail } => tail.str_loop(format!("{}, {}", acc, head)),
       }
   }
}

impl<T: fmt::Display> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test_linked_list {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(LinkedList::<u8>::empty(), LinkedList::Nil)
    }

    #[test]
    fn test_new_head() {
        assert_eq!(LinkedList::<u8>::new_head(10 as u8), LinkedList::<u8>::Tail { head: 10 as u8 })
    }

    #[test]
    fn test_append_from_empty() {
        let empty = LinkedList::<u8>::empty();
        let new = empty.append(10);
        assert_eq!(new, LinkedList::Tail { head: 10 });
    }

    #[test]
    fn test_append_from_tail() {
        let head = LinkedList::new_head(10 as u8);

        let expected = LinkedList::Link {
            head: 10 as u8,
            tail: Box::new(LinkedList::Tail { head: 20 as u8 }),
        };

        assert_eq!(head.append(20), expected);
    }

    #[test]
    fn test_append_from_link() {
        let list = LinkedList::Link {
            head: 10 as u8,
            tail: Box::new(LinkedList::Tail { head: 20 as u8 }),
        };
        
        let expected = LinkedList::Link {
            head: 10 as u8,
            tail: Box::new(
                LinkedList::Link {
                    head: 20 as u8,
                    tail: Box::new(LinkedList::Tail { head: 30 }),
                }
            ),
        };

        assert_eq!(list.append(30), expected);
    }
}

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
