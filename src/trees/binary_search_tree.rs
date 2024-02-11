use std::fmt::Display;
use crate::collections::list::{List, QueueStackMix};

struct BSTNode<T> where T : PartialOrd {
    value: T,
    smaller: Option<Box<BSTNode<T>>>,
    larger: Option<Box<BSTNode<T>>>,
}

struct BinarySearchTree<T>  where T : PartialOrd + Display {
    root: Option<Box<BSTNode<T>>>,
}

impl<T> BinarySearchTree<T> where T : PartialOrd + Display {
    fn new() -> Self {
        BinarySearchTree {
            root: None,
        }
    }

    pub fn add(&mut self, value: T) -> Result<(),()> {
        if self.root.is_none() {
            self.root = Self::new_node(value);
            return Ok(());
        }

        let node = self.root.as_mut().unwrap();
        return Self::internal_add(node, value);
    }

    fn new_node(value: T) -> Option<Box<BSTNode<T>>> {
        Some(Box::new(BSTNode {
            value,
            smaller: None,
            larger: None,
        }))
    }
    fn internal_add (node: &mut BSTNode<T>, value: T) -> Result<(),()> {
        if value < node.value {
            if node.smaller.is_none() {
                node.smaller = Self::new_node(value);
                return Ok(());
            }
            let new_node = node.smaller.as_mut().unwrap();
            return Self::internal_add(new_node, value);
        } else {
            if node.larger.is_none() {
                node.larger = Self::new_node(value);
                return Ok(());
            }
            let new_node = node.larger.as_mut().unwrap();
            return Self::internal_add(new_node, value);
        }
    }

    pub fn search(&self, value: T) -> Result<(),()> {
        if self.root.is_none() {
            return Err(());
        }
        return Self::internal_search(self.root.as_ref().unwrap(), value);
    }

    fn internal_search (node: &Box<BSTNode<T>>, value: T) -> Result<(),()> {
        if value == node.value {
            return Ok(());
        } else if value < node.value {
            if node.smaller.is_none() {
                return Err(());
            }
            return Self::internal_search(node.smaller.as_ref().unwrap(), value);
        } else {
            if node.larger.is_none() {
                return Err(());
            }
            return Self::internal_search(node.larger.as_ref().unwrap(), value);
        }
    }

    pub fn dfs_print(&self) {
        Self::dfs(&self.root);
    }
    fn dfs(node: &Option<Box<BSTNode<T>>>) {
        if node.is_none() {
            return;
        }

        node.as_ref().map(|n| {
            Self::dfs(&n.smaller);
            println!("{}", n.value);
            Self::dfs(&n.larger);
        });
    }

    pub fn bfs_print(&self) {
        if self.root.is_none() {
            return;
        }

        let mut l = List::<&Option<Box<BSTNode<T>>>>::new();
        l.push_to_end(&self.root).unwrap();
        loop {
            match l.pop_from_beginning() {
                None => return,
                Some(n) => {
                    if n.is_none() {
                        continue;
                    }
                    let node = n.as_ref().unwrap();
                    println!("{}", node.value);
                    l.push_to_end(&node.smaller).unwrap();
                    l.push_to_end(&node.larger).unwrap();
                }
            }
        }
    }
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn add_test() {
        let bst = create_tree();

        assert_eq!(bst.search(5), Ok(()));
        assert_eq!(bst.search(6), Ok(()));
        assert_eq!(bst.search(7), Ok(()));
        assert_eq!(bst.search(1), Ok(()));
        assert_eq!(bst.search(2), Ok(()));
        assert_eq!(bst.search(3), Ok(()));
        assert_eq!(bst.search(10), Ok(()));
        assert_eq!(bst.search(20), Err(()));
        assert_eq!(bst.search(30), Err(()));
        assert_eq!(bst.search(11), Err(()));
        assert_eq!(bst.search(4), Ok(()));
        assert_eq!(bst.search(5), Ok(()));
        assert_eq!(bst.search(10), Ok(()));
        assert_eq!(bst.search(20), Err(()));
    }

    #[test]
    fn test_dfs() {
        let bst = create_tree();
        bst.dfs_print();
    }

    #[test]
    fn test_bfs() {
        let bst = create_tree();
        bst.bfs_print();
    }

    fn create_tree() -> BinarySearchTree<i32> {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.add(5), Ok(()));
        assert_eq!(bst.add(7), Ok(()));
        assert_eq!(bst.add(2), Ok(()));
        assert_eq!(bst.add(8), Ok(()));
        assert_eq!(bst.add(1), Ok(()));
        assert_eq!(bst.add(6), Ok(()));
        assert_eq!(bst.add(3), Ok(()));
        assert_eq!(bst.add(4), Ok(()));
        assert_eq!(bst.add(9), Ok(()));
        assert_eq!(bst.add(10), Ok(()));
        bst
    }
}