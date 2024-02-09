struct BSTNode<T> where T : PartialOrd {
    value: T,
    smaller: Option<Box<BSTNode<T>>>,
    larger: Option<Box<BSTNode<T>>>,
}

struct BinarySearchTree<T>  where T : PartialOrd {
    root: Option<Box<BSTNode<T>>>,
}

impl<T> BinarySearchTree<T> where T : PartialOrd {
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

    fn search(&self, value: T) -> Result<(),()> {
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
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn add_test() {
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
}