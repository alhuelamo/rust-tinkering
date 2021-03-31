use std::fmt;
use std::string::ToString;

#[derive(PartialEq)]
pub enum LinkedList<T: fmt::Display> {
    Nil,
    Tail { head: T },
    Link { head: T, tail: Box<LinkedList<T>> },
}

impl<T: fmt::Display> LinkedList<T> {
    pub fn empty() -> Self {
        LinkedList::Nil
    }

    pub fn new(element: T) -> Self {
        LinkedList::Tail { head: element }
    }

    pub fn append(self, element: T) -> Self {
        match self {
            LinkedList::Nil => LinkedList::new(element),
            LinkedList::Tail { head } => {
                let new_tail = Box::new(LinkedList::new(element));
                LinkedList::Link {
                    head,
                    tail: new_tail,
                }
            }
            LinkedList::Link { head, tail } => {
                let new_tail = Box::new(tail.append(element));
                LinkedList::Link {
                    head,
                    tail: new_tail,
                }
            }
        }
    }

    pub fn peek(&self, index: usize) -> Result<&T, &'static str> {
        match self {
            LinkedList::Nil => Err("Cannot peek anything from an empty list."),
            LinkedList::Tail { head } if index == 0 => Ok(head),
            LinkedList::Tail { .. } => Err("Index overflow."),
            LinkedList::Link { head, .. } if index == 0 => Ok(head),
            LinkedList::Link { tail, ..} => tail.peek(index - 1),
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
        assert_eq!(
            LinkedList::<u8>::new(10 as u8),
            LinkedList::<u8>::Tail { head: 10 as u8 }
        )
    }

    #[test]
    fn test_append_from_empty() {
        let empty = LinkedList::<u8>::empty();
        let new = empty.append(10);
        assert_eq!(new, LinkedList::Tail { head: 10 });
    }

    #[test]
    fn test_append_from_tail() {
        let head = LinkedList::new(10 as u8);

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
            tail: Box::new(LinkedList::Link {
                head: 20 as u8,
                tail: Box::new(LinkedList::Tail { head: 30 }),
            }),
        };

        assert_eq!(list.append(30), expected);
    }

    #[test]
    fn test_peek() {
        let list = LinkedList::<u8>::new(9 as u8).append(42);
        assert_eq!(list.peek(0).unwrap(), &(9 as u8));
        assert_eq!(list.peek(1).unwrap(), &(42 as u8));
    }

    #[test]
    fn test_peek_nil() {
        let list = LinkedList::<u8>::Nil;
        assert_eq!(list.peek(0), Err("Cannot peek anything from an empty list."))
    }

    #[test]
    fn test_peek_overflow() {
        let list = LinkedList::new(9).append(42);
        assert_eq!(list.peek(2), Err("Index overflow."))
    }
}
