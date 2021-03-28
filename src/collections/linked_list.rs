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
