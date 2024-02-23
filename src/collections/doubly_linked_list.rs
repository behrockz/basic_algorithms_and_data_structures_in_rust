use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    value: T,
    next: Link<T>,
    previous: Link<T>,
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    fn insert_by_index(&mut self, index: i32, value: T) -> Result<(), ()> {
        if index > self.length as i32 || (index * -1) > (self.length + 1) as i32 {
            return Err(());
        }

        let mut node = Node {
            value,
            next: None,
            previous: None,
        };

        if index == 0 || index+1 == (-1 * self.length as i32) {
            if self.head.is_none() {
                self.head = Some(Rc::new(RefCell::new(node)));
                self.tail = self.head.clone();
            } else {
                self.head.take().map(|old_head| {
                    node.next = Some(old_head.clone());
                    let rc = Rc::new(RefCell::new(node));
                    old_head.borrow_mut().previous = Some(rc.clone());
                    self.head = Some(rc.clone());
                });
            }
        } else if index == -1 || index == self.length as i32 {
            if self.tail.is_none() {
                self.tail = Some(Rc::new(RefCell::new(node)));
                self.head = self.tail.clone();
            } else {
                self.tail.take().map(|old_tail| {
                    node.previous = Some(old_tail.clone());
                    let rc = Rc::new(RefCell::new(node));
                    old_tail.borrow_mut().next = Some(rc.clone());
                    self.tail = Some(rc.clone());
                });
            }
        } else if index >= 1 {
            let mut iter = self.head.clone();
            for _ in 1..index {
                iter = iter.unwrap().borrow_mut().next.clone();
            }
            iter.take().map(|n| {
                node.next = n.borrow().next.clone();
                node.previous = Some(n.clone());
                let rc = Rc::new(RefCell::new(node));
                if n.borrow_mut().next.is_some() {
                    n.borrow_mut().next.as_ref().unwrap().borrow_mut().previous = Some(rc.clone());
                }
                n.borrow_mut().next = Some(rc.clone());
            });
        } else if index < 0 {
            let mut iter = self.tail.clone();
            for _ in 2..(-1 * index) {
                iter = iter.unwrap().borrow_mut().previous.clone();
            }
            iter.take().map(|n| {
                node.next = Some(n.clone());
                node.previous = n.borrow().previous.clone();
                let rc = Rc::new(RefCell::new(node));

                if n.borrow_mut().previous.is_some() {
                    n.borrow_mut().previous.as_ref().unwrap().borrow_mut().next = Some(rc.clone());
                }
                n.borrow_mut().previous = Some(rc.clone());
            });
        }
        self.length += 1;
        Ok(())
    }
    fn get_by_index(&mut self, index: i32) -> Link<T> {
        if index > self.length as i32 || (index * -1) > (self.length + 1) as i32 {
            return None;
        }

        let mut result : Link<T> = None;

        if index == 0 || index == (-1 * self.length as i32) {
            if self.head.is_none() {
                result = None;
            } else {
                self.head.take().map(|old_head| {
                    self.head = old_head.borrow().next.clone();
                    if self.head.is_some() {
                        self.head.clone().unwrap().borrow_mut().previous = None;
                    } else {
                        self.tail = None;
                    }

                    result = Some(old_head);
                });
            }
        } else if index == -1 || index == self.length as i32 {
            if self.tail.is_none() {
                result = None;
            } else {
                self.tail.take().map(|old_tail| {
                    self.tail = old_tail.borrow().previous.clone();
                    if self.tail.is_some() {
                        self.tail.clone().unwrap().borrow_mut().next = None;
                    } else {
                        self.head = None;
                    }

                    result = Some(old_tail);
                });
            }
        } else if index >= 1 {
            let mut iter = self.head.clone();
            for _ in 0..index {
                iter = iter.unwrap().borrow_mut().next.clone();
            }
            iter.take().map(|n| {
                n.borrow().previous.clone().unwrap().borrow_mut().next = n.borrow().next.clone();
                n.borrow().next.clone().unwrap().borrow_mut().previous = n.borrow().previous.clone();

                result = Some(n);
            });
        } else if index < 0 {
            let mut iter = self.tail.clone();
            for _ in 1..(-1 * index) {
                iter = iter.unwrap().borrow_mut().previous.clone();
            }
            iter.take().map(|n| {
                n.borrow().previous.clone().unwrap().borrow_mut().next = n.borrow().next.clone();
                n.borrow().next.clone().unwrap().borrow_mut().previous = n.borrow().previous.clone();

                result = Some(n);
            });
        }

        if result.is_some() {
            self.length -= 1;
        }
        result
    }

    fn get_first(&mut self) -> Link<T> {
        self.get_by_index(0)
    }

    fn get_last(&mut self) -> Link<T> {
        self.get_by_index(-1)
    }

    fn insert_first(&mut self, value: T) -> Result<(), ()> {
        self.insert_by_index(0, value)
    }

    fn insert_last(&mut self, value: T) -> Result<(), ()> {
        self.insert_by_index(-1, value)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut iter = self.head.take();
        self.tail.take();

        while let Some(i) = iter {
            i.borrow_mut().previous.take();
            iter = i.borrow_mut().next.take();
            drop(i);
        }

    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn new_list() {
        let mut list = DoublyLinkedList::new();

        assert_eq!(list.insert_by_index(0, 3), Ok(()));
        assert_eq!(list.insert_first( 2), Ok(()));
        assert_eq!(list.insert_by_index(0, 1), Ok(()));

        assert_eq!(list.insert_by_index(3, 4), Ok(()));
        assert_eq!(list.insert_last( 5), Ok(()));
        assert_eq!(list.insert_by_index(5, 6), Ok(()));

        assert_eq!(list.get_length(), 6);

        assert_eq!(list.insert_by_index(-1, 8), Ok(()));
        assert_eq!(list.insert_by_index(-2, 7), Ok(()));

        assert_eq!(list.get_length(), 8);

        assert_eq!(list.get_by_index(0).unwrap().borrow().value, 1);
        assert_eq!(list.get_by_index(1).unwrap().borrow().value, 3);
        assert_eq!(list.get_first().unwrap().borrow().value, 2);

        assert_eq!(list.get_length(), 5);

        assert_eq!(list.get_by_index(-1).unwrap().borrow().value, 8);
        assert_eq!(list.get_by_index(-2).unwrap().borrow().value, 6);

        assert_eq!(list.get_length(), 3);

        assert_eq!(list.get_by_index(-3).unwrap().borrow().value, 4);
        assert_eq!(list.get_by_index(2).unwrap().borrow().value, 7);
        assert_eq!(list.get_by_index(0).unwrap().borrow().value, 5);

        assert_eq!(list.get_length(), 0);

        assert!(list.get_first().is_none());

        assert_eq!(list.insert_by_index(0, 3), Ok(()));
        assert_eq!(list.insert_first( 2), Ok(()));
        assert_eq!(list.insert_by_index(0, 1), Ok(()));

        drop(list);

    }
    #[test]
    fn new_list_reserve() {
        let mut list = DoublyLinkedList::new();

        assert_eq!(list.insert_by_index(-1, 1), Ok(()));
        assert_eq!(list.insert_by_index(-1, 2), Ok(()));
        assert_eq!(list.insert_by_index(-1, 4), Ok(()));
        assert_eq!(list.insert_by_index(-2, 3), Ok(()));
        assert_eq!(list.insert_by_index(4, 7), Ok(()));
        assert_eq!(list.insert_by_index(4, 6), Ok(()));
        assert_eq!(list.insert_by_index(-3, 5), Ok(()));
        assert_eq!(list.insert_by_index(-8, 0), Ok(()));
        assert_eq!(list.insert_by_index(-9, -1), Ok(()));
        assert_eq!(list.insert_by_index(-10, -2), Ok(()));
        assert_eq!(list.insert_by_index(10, 8), Ok(()));
        assert_eq!(list.insert_by_index(-19, 8), Err(()));
        assert_eq!(list.insert_by_index(19, 8), Err(()));

        assert_eq!(list.get_first().unwrap().borrow().value, -2);
        assert_eq!(list.get_first().unwrap().borrow().value, -1);
        assert_eq!(list.get_last().unwrap().borrow().value, 8);
        assert_eq!(list.get_last().unwrap().borrow().value, 7);

        assert_eq!(list.get_by_index(-7).unwrap().borrow().value, 0);
        assert_eq!(list.get_by_index(6).unwrap().borrow().value, 6);

        assert_eq!(list.get_by_index(2).unwrap().borrow().value, 3);
        assert_eq!(list.get_by_index(-3).unwrap().borrow().value, 2);
        assert_eq!(list.get_by_index(1).unwrap().borrow().value, 4);
        assert_eq!(list.get_by_index(-2).unwrap().borrow().value, 1);
        assert_eq!(list.get_by_index(0).unwrap().borrow().value, 5);
    }
}