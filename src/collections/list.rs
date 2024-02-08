struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            length: 0,
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn insert_by_index(&mut self, index: usize, value: T) -> Result<(), ()> {
        if index > self.length + 1 {
            return Err(());
        }

        self.length += 1;

        if index == 0 {
            self.head = Some(Box::new(Node {
                value,
                next: self.head.take(),
            }));
            return Ok(());
        }

        let mut iter = self.head.as_mut().unwrap().as_mut();

        for _ in 0..index - 1 {
            iter = iter.next.as_mut().unwrap().as_mut();
        }

        let old_next = iter.next.take();
        iter.next = Some(Box::new(Node {
            value,
            next: old_next,
        }));
        return Ok(());
    }

    fn get_by_index(&mut self, index: usize) -> Option<T> {
        if index > self.length {
            return None;
        }

        if self.length != 0 {
            self.length -= 1;
        }

        if index == 0 || self.length == 0 {
            let old_head = self.head.take();
            return old_head.map(|n| {
                self.head = n.next;
                n.value
            });
        }

        let mut prev = self.head.as_mut().unwrap().as_mut();
        for _ in 0..index - 1 {
            prev = prev.next.as_mut().unwrap().as_mut();
        }

        let target = prev.next.take();

        target.map(|n| {
            prev.next = n.next;
            n.value
        })
    }

    fn peek_by_index(&self, index: usize) -> Option<&T> {
        if index > self.length {
            return None;
        }

        if index == 0 {
            return self.head.as_ref().map(|n| &n.value);
        }

        let mut iter = self.head.as_ref().unwrap().as_ref();
        for _ in 0..index {
            iter = iter.next.as_ref().unwrap().as_ref();
        }

        Some(&iter.value)
    }

    fn peek_mut_by_index(&mut self, index: usize) -> Option<&mut T> {
        if index > self.length {
            return None;
        }

        if index == 0 {
            return self.head.as_mut().map(|n| &mut n.value);
        }

        let mut prev = self.head.as_mut().unwrap().as_mut();
        for _ in 0..index {
            prev = prev.next.as_mut().unwrap().as_mut();
        }

        Some(&mut prev.value)
    }
}

pub trait QueueStackMix<T> {
    fn push_to_beginning(&mut self, value: T) -> Result<(), ()>;
    fn pop_from_beginning(&mut self) -> Option<T>;
    fn push_to_end(&mut self, value: T) -> Result<(), ()>;
    fn pop_from_end(&mut self) -> Option<T>;
    fn peek_beginning(&self) -> Option<&T>;
    fn peek_last(&self) -> Option<&T>;
    fn peek_mut_beginning(&mut self) -> Option<&mut T>;
    fn peek_mut_last(&mut self) -> Option<&mut T>;
}

impl<T> QueueStackMix<T> for List<T> {
    fn push_to_beginning(&mut self, value: T) -> Result<(), ()> {
        self.insert_by_index(0, value)
    }
    fn pop_from_beginning(&mut self) -> Option<T> {
        self.get_by_index(0)
    }
    fn push_to_end(&mut self, value: T) -> Result<(), ()> {
        self.insert_by_index(self.length, value)
    }
    fn pop_from_end(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.get_by_index(self.length - 1)
    }
    fn peek_beginning(&self) -> Option<&T> {
        self.peek_by_index(0)
    }
    fn peek_last(&self) -> Option<&T> {
        if self.length == 0 {
            return None;
        }
        self.peek_by_index(self.length - 1)
    }
    fn peek_mut_beginning(&mut self) -> Option<&mut T> {
        self.peek_mut_by_index(0)
    }
    fn peek_mut_last(&mut self) -> Option<&mut T> {
        if self.length == 0 {
            return None;
        }
        self.peek_mut_by_index(self.length - 1)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut iter = self.head.take();

        while let Some(mut i) = iter {
            iter = i.next.take();
        }
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_from_beginning()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn new_list() {
        let mut l: List<u32> = List::new();
        assert_eq!(l.get_length(), 0);
        assert_eq!(l.pop_from_beginning(), None);
        assert_eq!(l.pop_from_end(), None);
        assert_eq!(l.peek_beginning(), None);
        assert_eq!(l.peek_last(), None);
        assert_eq!(l.peek_mut_beginning(), None);
        assert_eq!(l.peek_mut_last(), None);
    }

    #[test]
    fn push_pop_beginning() {
        let mut l: List<u32> = List::new();
        assert!(l.push_to_beginning(4).is_ok());
        assert!(l.push_to_beginning(3).is_ok());
        assert!(l.push_to_beginning(2).is_ok());
        assert!(l.push_to_beginning(1).is_ok());
        assert_eq!(l.get_length(), 4);
        assert_eq!(l.pop_from_beginning(), Some(1));
        assert_eq!(l.pop_from_beginning(), Some(2));
        assert_eq!(l.pop_from_beginning(), Some(3));
        assert_eq!(l.pop_from_beginning(), Some(4));
        assert_eq!(l.pop_from_beginning(), None);
        assert_eq!(l.get_length(), 0);
        assert!(l.push_to_beginning(5).is_ok());
        assert!(l.push_to_beginning(6).is_ok());
        assert_eq!(l.get_length(), 2);
        assert_eq!(l.pop_from_beginning(), Some(6));
        assert_eq!(l.pop_from_beginning(), Some(5));
        assert_eq!(l.pop_from_beginning(), None);
        assert_eq!(l.get_length(), 0);
    }

    #[test]
    fn push_pop_end() {
        let mut l: List<u32> = List::new();
        assert!(l.push_to_end(4).is_ok());
        assert!(l.push_to_end(3).is_ok());
        assert!(l.push_to_end(2).is_ok());
        assert!(l.push_to_end(1).is_ok());
        assert_eq!(l.get_length(), 4);
        assert_eq!(l.pop_from_end(), Some(1));
        assert_eq!(l.pop_from_end(), Some(2));
        assert_eq!(l.pop_from_end(), Some(3));
        assert_eq!(l.pop_from_end(), Some(4));
        assert_eq!(l.pop_from_end(), None);
        assert_eq!(l.get_length(), 0);
        assert!(l.push_to_beginning(5).is_ok());
        assert!(l.push_to_beginning(6).is_ok());
        assert_eq!(l.get_length(), 2);
        assert_eq!(l.pop_from_beginning(), Some(6));
        assert_eq!(l.pop_from_beginning(), Some(5));
        assert_eq!(l.pop_from_beginning(), None);
        assert_eq!(l.get_length(), 0);
    }

    #[test]
    fn mixed_push_pop_end_beginning() {
        let mut l: List<u32> = List::new();
        assert!(l.push_to_beginning(2).is_ok());
        assert!(l.push_to_beginning(1).is_ok());
        assert!(l.push_to_end(3).is_ok());
        assert!(l.push_to_end(4).is_ok());
        assert_eq!(l.get_length(), 4);
        assert_eq!(l.pop_from_end(), Some(4));
        assert_eq!(l.pop_from_beginning(), Some(1));
        assert_eq!(l.pop_from_end(), Some(3));
        assert_eq!(l.pop_from_beginning(), Some(2));
        assert_eq!(l.pop_from_end(), None);
        assert_eq!(l.get_length(), 0);
        assert!(l.push_to_beginning(5).is_ok());
        assert!(l.push_to_end(6).is_ok());
        assert_eq!(l.get_length(), 2);
        assert_eq!(l.pop_from_beginning(), Some(5));
        assert_eq!(l.pop_from_beginning(), Some(6));
        assert_eq!(l.pop_from_beginning(), None);
        assert_eq!(l.get_length(), 0);
    }

    #[test]
    fn mixed_get_insert() {
        let mut l: List<u32> = List::new();
        assert!(l.insert_by_index(0, 4).is_ok());
        assert!(l.insert_by_index(0, 1).is_ok());
        assert!(l.insert_by_index(1, 3).is_ok());
        assert!(l.insert_by_index(1, 2).is_ok());
        assert!(l.insert_by_index(4, 5).is_ok());
        assert_eq!(l.get_length(), 5);
        assert_eq!(l.get_by_index(2), Some(3));
        assert_eq!(l.get_by_index(2), Some(4));
        assert_eq!(l.get_by_index(2), Some(5));
        assert_eq!(l.get_by_index(0), Some(1));
        assert_eq!(l.get_by_index(0), Some(2));
        assert_eq!(l.pop_from_end(), None);
        assert_eq!(l.get_length(), 0);
        assert_eq!(l.insert_by_index(5, 0), Err(()));
        assert_eq!(l.get_length(), 0);

        assert!(l.insert_by_index(0, 6).is_ok());
        assert!( l.insert_by_index(0, 7).is_ok());
        assert_eq!(l.get_length(), 2);
        assert_eq!(l.pop_from_beginning(), Some(7));
        assert_eq!(l.pop_from_beginning(), Some(6));
        assert_eq!(l.pop_from_beginning(), None);
        assert_eq!(l.get_length(), 0);
    }

    #[test]
    fn iter() {
        let mut l: List<u32> = List::new();
        assert!(l.push_to_beginning(2).is_ok());
        assert!(l.push_to_beginning(1).is_ok());
        assert!(l.push_to_end(4).is_ok());
        assert!(l.push_to_end(5).is_ok());
        assert!(l.insert_by_index(2, 3).is_ok());

        assert_eq!(l.into_iter().sum::<u32>(), 15);
    }

    #[test]
    fn peak() {
        let mut l: List<u32> = List::new();
        assert!(l.push_to_beginning(2).is_ok());
        assert!(l.push_to_beginning(1).is_ok());
        assert!(l.push_to_end(4).is_ok());
        assert!(l.push_to_end(5).is_ok());
        assert!(l.insert_by_index(2, 3).is_ok());

        assert_eq!(l.peek_beginning(), Some(&1));
        assert_eq!(l.peek_last(), Some(&5));
        assert_eq!(l.peek_by_index(1), Some(&2));
        assert_eq!(l.peek_by_index(2), Some(&3));
        assert_eq!(l.peek_by_index(3), Some(&4));
    }

    #[test]
    fn peak_mut() {
        let mut l: List<u32> = List::new();
        assert!(l.push_to_beginning(2).is_ok());
        assert!(l.push_to_beginning(1).is_ok());
        assert!(l.push_to_end(4).is_ok());
        assert!(l.push_to_end(5).is_ok());
        assert!(l.insert_by_index(2, 3).is_ok());
        assert_eq!(l.get_length(), 5);

        l.peek_mut_last().map(|v| *v = 10);
        l.peek_mut_beginning().map(|v| *v = 6);
        l.peek_mut_by_index(1).map(|v| *v = 7);
        l.peek_mut_by_index(2).map(|v| *v = 8);
        l.peek_mut_by_index(3).map(|v| *v = 9);
        assert_eq!(l.get_length(), 5);

        assert_eq!(l.pop_from_beginning(), Some(6));
        assert_eq!(l.pop_from_end(), Some(10));
        assert_eq!(l.get_by_index(1), Some(8));
        assert_eq!(l.get_by_index(1), Some(9));
        assert_eq!(l.get_by_index(0), Some(7));
        assert_eq!(l.get_length(), 0);
    }
}

