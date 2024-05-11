use bitvec::prelude::*;
use std::mem::swap;
use rand::prelude::*;

type Key = BitVec<u8, Msb0>;
type KeySlice = BitSlice<u8, Msb0>;

#[derive(Debug)]
 struct LoopyPatriciaNode {
    left_child: Option<Box<LoopyPatriciaNode>>,
    right_child: Option<Box<LoopyPatriciaNode>>,
    key: Key,
    end: bool
}

impl LoopyPatriciaNode {
    fn new(k: &KeySlice, end: bool) -> Self {
        let mut key = BitVec::new();
        key.extend_from_bitslice(k);
        LoopyPatriciaNode {
            left_child: None,
            right_child: None,
            end,
            key,
        }
    }
}

#[derive(Debug)]
pub(crate) struct LoopyPatriciaTree {
    root: Option<Box<LoopyPatriciaNode>>,
}

impl LoopyPatriciaTree {
    pub fn new() -> Self {
        LoopyPatriciaTree {
            root: None,
        }
    }

    pub fn insert(&mut self, original_key: &Key) {
        if self.root.is_none() {
            self.root = Some(Box::new(LoopyPatriciaNode::new(original_key.as_bitslice(), true)));
            return;
        }

        let mut node =  self.root.as_mut().unwrap();
        let mut key = original_key.as_bitslice();

        loop{
            let key_len = key.len();
            let i = Self::find_number_of_matching_bits(&node.key, &key);
            let (base_segment, new_segment) = key.split_at(i);
            let (_, old_segment) = node.key.split_at(i);

            if i == key_len {
                if i!= 0 && i != node.key.len() {
                    let mut new_child = LoopyPatriciaNode::new(old_segment, node.end);
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
                        node = node.right_child.as_mut().unwrap();
                        key = new_segment;
                        continue;
                    } else {
                        node.right_child = Some(Box::new(LoopyPatriciaNode::new(new_segment, true)));
                    }
                } else {
                    if node.left_child.is_some() {
                        node = node.left_child.as_mut().unwrap();
                        key = new_segment;
                        continue;
                    } else {
                        node.left_child = Some(Box::new(LoopyPatriciaNode::new(new_segment, true)));
                    }
                    return;
                }
            } else {
                let mut new_right_node;
                let mut new_left_node;

                if key[i] == false {
                    new_left_node = LoopyPatriciaNode::new(new_segment, true);
                    new_right_node = LoopyPatriciaNode::new(old_segment, node.end);
                    swap(&mut new_right_node.left_child, &mut node.left_child);
                    swap(&mut new_right_node.right_child, &mut node.right_child);
                } else {
                    new_left_node = LoopyPatriciaNode::new(old_segment, node.end);
                    swap(&mut new_left_node.left_child, &mut node.left_child);
                    swap(&mut new_left_node.right_child, &mut node.right_child);
                    new_right_node = LoopyPatriciaNode::new(new_segment, true);
                }
                node.right_child = Some(Box::new(new_right_node));
                node.left_child = Some(Box::new(new_left_node));
                node.key.clear();
                node.key.extend_from_bitslice(base_segment);
                node.end = false;
            }
            return;
        }
    }

    fn find_number_of_matching_bits(src: &Key, dest: &KeySlice) -> usize {
        let length = src.len().min(dest.len());
        for i in 0..length {
            if src[i] != dest[i] {
                return i;
            }
        }
        return length;
    }

    pub fn search(&self, original_key: &Key) -> bool {
        if self.root.is_none() { return false; }

        let mut node =  self.root.as_ref().unwrap();
        let mut key = original_key.as_bitslice();

        loop {
            let i = Self::find_number_of_matching_bits(&node.key, &key);
            if i == key.len() {
                return node.end
            }

            let (_, right) = key.split_at(i);

            if right[0] == true {
                if node.right_child.is_none() {
                    return false;
                }
                node = node.right_child.as_ref().unwrap();
                key = right;
                continue;
            } else {
                if node.left_child.is_none() {
                    return false;
                }
                node = node.left_child.as_ref().unwrap();
                key = right;
            }
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
        let mut rng = rand::thread_rng();

        const SIZE : usize = 4096;
        let mut p = crate::collections::loopy_patricia::LoopyPatriciaTree::new();
        let mut vector = Vec::new();

        for _i in 0..100 {
            let mut bytes: [u8; SIZE] = [0u8; SIZE];
            rng.fill_bytes(&mut bytes);
            let b = bytes.view_bits::<Msb0>().to_bitvec();
            p.insert(&b);
            // assert!(p.search(&b));
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


    fn dfs(p: &LoopyPatriciaNode, s: &str) {
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

