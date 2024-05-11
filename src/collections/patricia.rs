use bitvec::prelude::*;
use std::mem::swap;


type Key = BitVec<u8, Msb0>;
type KeySlice = BitSlice<u8, Msb0>;

#[derive(Debug)]
struct PatriciaNode {
    left_child: Option<Box<PatriciaNode>>,
    right_child: Option<Box<PatriciaNode>>,
    key: Key,
    end: bool
}

impl PatriciaNode {
    fn new(key: &KeySlice, end: bool) -> Self {
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
pub(crate) struct PatriciaTree {
    root: PatriciaNode,
}

impl PatriciaTree {
    pub fn new() -> Self {
        PatriciaTree {
            root: PatriciaNode::new(&BitVec::new().as_bitslice(), false),
        }
    }

    pub fn insert(&mut self, key: &Key) {
        Self::find_and_insert(&mut self.root, key);
    }

    fn find_and_insert(node: &mut PatriciaNode, key: &Key) {
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
                node.key.truncate(i);
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

    fn find_number_of_matching_bits(src: &Key, dest: &Key) -> usize {
        let length = src.len().min(dest.len());
        for i in 0..length {
            if src[i] != dest[i] {
                return i;
            }
        }
        return length;
    }

    // Search for a key in the tree
    pub fn search(&self, key: &Key) -> bool {
        Self::search_internal(&self.root, key)
    }

    // Search for a key in the tree
    fn search_internal(node: &PatriciaNode, key: &Key) -> bool {
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
    use rand::RngCore;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn add_test() {
        let mut rng = rand::thread_rng();

        const SIZE : usize = 4096;
        let mut p = crate::collections::patricia::PatriciaTree::new();
        let mut vector = Vec::new();

        for _i in 0..100 {
            let mut bytes: [u8; SIZE] = [0u8; SIZE];
            rng.fill_bytes(&mut bytes);
            let b = bytes.view_bits::<Msb0>().to_bitvec();
            p.insert(&b);
            vector.push(b);
        }

        for b in vector {
            assert!(p.search(&b));
        }

        for _i in 0..100 {
            let mut bytes: [u8; SIZE] = [0u8; SIZE];
            rng.fill_bytes(&mut bytes);
            let b = bytes.view_bits::<Msb0>().to_bitvec();
            assert!(!p.search(&b));
        }
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

