use bitvec::prelude::*;
use std::mem::swap;

#[derive(Debug)]
struct PatriciaNode {
    left_child: Option<Box<PatriciaNode>>,
    right_child: Option<Box<PatriciaNode>>,
    key: BitVec,
    end: bool
}

impl PatriciaNode {
    fn new(key: &BitSlice, end: bool) -> Self {
        let mut new_key = BitVec::new();
        new_key.extend_from_bitslice(key);
        PatriciaNode {
            left_child: None,
            right_child: None,
            end,
            key: new_key,
        }
    }
}

#[derive(Debug)]
struct PatriciaTree {
    root: PatriciaNode,
}

impl PatriciaTree {
    pub fn new() -> Self {
        PatriciaTree {
            root: PatriciaNode::new(&BitVec::new().as_bitslice(), false),
        }
    }

    pub fn insert(&mut self, key: &BitVec) {
        Self::find_and_insert(&mut self.root, key);
    }

    fn find_and_insert(node: &mut PatriciaNode, key: &BitVec) {
        let i = Self::find_number_of_matching_bits(&node.key, &key);

        let (base_segment, new_segment) = key.split_at(i);
        let (_, old_segment) = node.key.split_at(i);

        if i == key.len() {
            if i!= 0 && i != node.key.len() {
                let mut new_child = PatriciaNode::new(&old_segment.to_bitvec(), node.end);
                swap(&mut new_child.left_child, &mut node.left_child);
                swap(&mut new_child.right_child, &mut node.right_child);
                if old_segment[0] {
                    node.right_child = Some(Box::new(new_child));
                } else {
                    node.left_child = Some(Box::new(new_child));
                }
                node.key.clear();
                node.key.extend_from_bitslice(base_segment);
                node.end = true;
            }
            return;
        }

        if i == node.key.len() {
            if key[i] {
                if node.right_child.is_some() {
                    Self::find_and_insert(node.right_child.as_mut().unwrap(), &new_segment.to_bitvec());
                } else {
                    node.right_child = Some(Box::new(PatriciaNode::new(&new_segment.to_bitvec(), true)));
                }
            } else {
                if node.left_child.is_some() {
                    Self::find_and_insert(node.left_child.as_mut().unwrap(), &new_segment.to_bitvec());
                } else {
                    node.left_child = Some(Box::new(PatriciaNode::new(&new_segment.to_bitvec(), true)));
                }
            }
        } else {
            let mut new_right_node;
            let mut new_left_node;

            if key[i] == false {
                new_left_node = PatriciaNode::new(&new_segment.to_bitvec(), true);
                new_right_node = PatriciaNode::new(&old_segment.to_bitvec(), node.end);
                swap(&mut new_right_node.left_child, &mut node.left_child);
                swap(&mut new_right_node.right_child, &mut node.right_child);
            } else {
                new_left_node = PatriciaNode::new(&old_segment.to_bitvec(), node.end);
                swap(&mut new_left_node.left_child, &mut node.left_child);
                swap(&mut new_left_node.right_child, &mut node.right_child);
                new_right_node = PatriciaNode::new(&new_segment.to_bitvec(), true);
            }
            node.right_child = Some(Box::new(new_right_node));
            node.left_child = Some(Box::new(new_left_node));
            node.key.clear();
            node.key.extend_from_bitslice(base_segment);
            node.end = false;
        }
    }

    fn find_number_of_matching_bits(src: &BitVec, dest: &BitVec) -> usize {
        let length = src.len().min(dest.len());
        for i in 0..length {
            if src[i] != dest[i] {
                return i;
            }
        }
        return length;
    }


    // Search for a key in the tree
    pub fn search(&self, key: &BitVec) -> bool {
        Self::search_internal(&self.root, key)
    }

    // Search for a key in the tree
    fn search_internal(node: &PatriciaNode, key: &BitVec) -> bool {
        let i = Self::find_number_of_matching_bits(&node.key, key);
        if i == key.len() {
            return node.end
        }

        let (_, right) = key.split_at(i);



        if right[0] == true {
            if node.right_child.is_none() {
                return false;
            }
            Self::search_internal(node.right_child.as_ref().unwrap(), &right.to_bitvec())
        } else {
            if node.left_child.is_none() {
                return false;
            }
            Self::search_internal(node.left_child.as_ref().unwrap(), &right.to_bitvec())
        }
    }

    // fn delete(&mut self, _key: &BitVec) {
    //     unimplemented!(); // Implement delete method if needed
    // }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn add_test() {
        let mut p = PatriciaTree::new();

        let b1 = bitvec![0, 0, 0, 0, 0, 0, 1, 0];
        p.insert(&b1);
        println!("-----------------");
        dfs(&p.root, "");

        let b2 = bitvec![0, 0, 0, 0, 1, 0, 0, 0];
        p.insert(&b2);
        println!("-----------------");
        dfs(&p.root, "");

        let b3 = bitvec![0, 0, 0, 0, 0, 1, 0, 0];
        p.insert(&b3);
        println!("-----------------");
        dfs(&p.root, "");

        let b4 = bitvec![0, 0, 0, 0, 0, 1, 1, 1];
        p.insert(&b4);
        println!("-----------------");
        dfs(&p.root, "");

        let b5 = bitvec![0, 0, 0, 0, 0, 0, 0, 1];
        p.insert(&b5);
        println!("-----------------");
        dfs(&p.root, "");

        let b6 = bitvec![0, 0, 0, 0, 1];
        p.insert(&b6);
        println!("-----------------");
        dfs(&p.root, "");

        let b7 = bitvec![0, 0, 0, 0, 1, 0, 1, 0];
        p.insert(&b7);
        println!("-----------------");
        dfs(&p.root, "");

        let b8 = bitvec![0, 0, 0, 0, 1, 1, 0, 0];
        p.insert(&b8);
        println!("--------wrong---------");
        dfs(&p.root, "");

        let b9 = bitvec![0, 0, 0, 0, 0, 0, 1, 1];
        p.insert(&b9);
        println!("-----------------");
        dfs(&p.root, "");

        let b10 = bitvec![0, 0, 0, 0, 1, 1, 1, 1];
        p.insert(&b10);
        println!("-----------------");
        dfs(&p.root, "");

        assert!(p.search(&b1));
        assert!(p.search(&b2));
        assert!(p.search(&b3));
        assert!(p.search(&b4));
        assert!(p.search(&b5));
        assert!(p.search(&b6));
        assert!(p.search(&b7));
        assert!(p.search(&b8));
        assert!(p.search(&b9));
        assert!(p.search(&b10));
        let b11 = bitvec![1, 0, 0, 0, 1, 1, 1, 1];
        assert!(!p.search(&b11));
        let b12 = bitvec![0, 0, 0, 0, 1, 1, 1, 0];
        assert!(!p.search(&b12));
        let b13 = bitvec![0, 0, 0, 0, 1, 0, 1, 1];
        assert!(!p.search(&b13));
        let b14 = bitvec![0, 0, 0, 0, 0, 0];
        assert!(!p.search(&b14));
    }

    fn dfs(p: &PatriciaNode, s: &str) {
        let text = format!("{}-{}{}",s, p.key, if p.end {"+(end)"} else { "" });
        if p.left_child.is_some() {
            dfs(p.left_child.as_ref().unwrap(), text.as_str());
        }
        if p.right_child.is_some() {
            dfs(p.right_child.as_ref().unwrap(), text.as_str());
        }

        if p.left_child.is_none() && p.right_child.is_none() {
            println!("{}", text);
        }
    }
}

