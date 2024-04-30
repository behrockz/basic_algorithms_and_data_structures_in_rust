use bitvec::prelude::*;
use std::mem::swap;
use rand::prelude::*;


#[derive(Debug)]
struct LoopyPatriciaNode {
    left_child: Option<Box<LoopyPatriciaNode>>,
    right_child: Option<Box<LoopyPatriciaNode>>,
    key: BitVec<u8>,
    end: bool
}

impl LoopyPatriciaNode {
    fn new(key: &BitSlice<u8>, end: bool) -> Self {
        let mut new_key = BitVec::new();
        new_key.extend_from_bitslice(key);
        LoopyPatriciaNode {
            left_child: None,
            right_child: None,
            end,
            key: new_key,
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
            root: Some(Box::new(LoopyPatriciaNode::new(&BitVec::new().as_bitslice(), false))),
        }
    }

    pub fn insert(&mut self, org_key: &BitVec<u8>) {
        let mut node =  self.root.as_mut().unwrap();
        let mut key : BitVec<u8> = BitVec::new();
        key.extend_from_bitslice(org_key);

        loop{
            let i = Self::find_number_of_matching_bits(&node.key, &key);
            let mut split : BitVec<u8> = BitVec::new();
            split.extend_from_bitslice(&key);
            let (base_segment, new_segment) = split.split_at(i);
            let (_, old_segment) = node.key.split_at(i);

            if i == key.len() {
                if i!= 0 && i != node.key.len() {
                    let mut new_child = LoopyPatriciaNode::new(&old_segment.to_bitvec(), node.end);
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
                        node = node.right_child.as_mut().unwrap();
                        key.clear();
                        key.extend_from_bitslice(new_segment);
                        continue;
                    } else {
                        node.right_child = Some(Box::new(LoopyPatriciaNode::new(&new_segment.to_bitvec(), true)));
                    }
                } else {
                    if node.left_child.is_some() {
                        node = node.left_child.as_mut().unwrap();
                        key.clear();
                        key.extend_from_bitslice(new_segment);
                        continue;
                    } else {
                        node.left_child = Some(Box::new(LoopyPatriciaNode::new(&new_segment.to_bitvec(), true)));
                    }
                    return;
                }
            } else {
                let mut new_right_node;
                let mut new_left_node;

                if key[i] == false {
                    new_left_node = LoopyPatriciaNode::new(&new_segment.to_bitvec(), true);
                    new_right_node = LoopyPatriciaNode::new(&old_segment.to_bitvec(), node.end);
                    swap(&mut new_right_node.left_child, &mut node.left_child);
                    swap(&mut new_right_node.right_child, &mut node.right_child);
                } else {
                    new_left_node = LoopyPatriciaNode::new(&old_segment.to_bitvec(), node.end);
                    swap(&mut new_left_node.left_child, &mut node.left_child);
                    swap(&mut new_left_node.right_child, &mut node.right_child);
                    new_right_node = LoopyPatriciaNode::new(&new_segment.to_bitvec(), true);
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

    fn find_number_of_matching_bits(src: &BitVec<u8>, dest: &BitVec<u8>) -> usize {
        let length = src.len().min(dest.len());
        for i in 0..length {
            if src[i] != dest[i] {
                return i;
            }
        }
        return length;
    }

    pub fn search(&self, org_key: &BitVec<u8>) -> bool {
        let mut node =  self.root.as_ref().unwrap();
        let mut key : BitVec<u8> = BitVec::new();
        key.extend_from_bitslice(org_key);
        loop {
            let i = Self::find_number_of_matching_bits(&node.key, &key);
            if i == key.len() {
                return node.end
            }

            let mut split : BitVec<u8> = BitVec::new();
            split.extend_from_bitslice(&key);
            let (_, right) = split.split_at(i);


            if right[0] == true {
                if node.right_child.is_none() {
                    return false;
                }
                node = node.right_child.as_ref().unwrap();
                key = BitVec::new();
                key.extend_from_bitslice(right);
                continue;
            } else {
                if node.left_child.is_none() {
                    return false;
                }
                node = node.left_child.as_ref().unwrap();
                key = BitVec::new();
                key.extend_from_bitslice(right);
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
            let b = bytes.view_bits::<Lsb0>().to_bitvec();
            p.insert(&b);
            assert!(p.search(&b));
            vector.push(b);
        }

        for b in vector {
            assert!(p.search(&b));
        }

        for _i in 0..100 {
            let mut bytes: [u8; SIZE] = [0u8; SIZE];
            rng.fill_bytes(&mut bytes);
            let b = bytes.view_bits::<Lsb0>().to_bitvec();
            assert!(!p.search(&b));
        }
    }

    // #[test]
    // fn add_test() {
    //     let mut p = PatriciaTree::new();
    //
    //     let b1: BitVec<u8> = bitvec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8].bits;
    //     p.insert(&b1);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b2: BitVec<u8>  = bitvec![0, 0, 0, 0, 1, 0, 0, 0];
    //     p.insert(&b2);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b3: BitVec<u8>  = bitvec![0, 0, 0, 0, 0, 1, 0, 0];
    //     p.insert(&b3);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b4: BitVec<u8>  = bitvec![0, 0, 0, 0, 0, 1, 1, 1];
    //     p.insert(&b4);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b5: BitVec<u8>  = bitvec![0, 0, 0, 0, 0, 0, 0, 1];
    //     p.insert(&b5);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b6: BitVec<u8>  = bitvec![0, 0, 0, 0, 1];
    //     p.insert(&b6);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b7: BitVec<u8>  = bitvec![0, 0, 0, 0, 1, 0, 1, 0];
    //     p.insert(&b7);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b8: BitVec<u8>  = bitvec![0, 0, 0, 0, 1, 1, 0, 0];
    //     p.insert(&b8);
    //     println!("--------wrong---------");
    //     dfs(&p.root, "");
    //
    //     let b9: BitVec<u8>  = bitvec![0, 0, 0, 0, 0, 0, 1, 1];
    //     p.insert(&b9);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     let b10: BitVec<u8>  = bitvec![0, 0, 0, 0, 1, 1, 1, 1];
    //     p.insert(&b10);
    //     println!("-----------------");
    //     dfs(&p.root, "");
    //
    //     assert!(p.search(&b1));
    //     assert!(p.search(&b2));
    //     assert!(p.search(&b3));
    //     assert!(p.search(&b4));
    //     assert!(p.search(&b5));
    //     assert!(p.search(&b6));
    //     assert!(p.search(&b7));
    //     assert!(p.search(&b8));
    //     assert!(p.search(&b9));
    //     assert!(p.search(&b10));
    //     let b11: BitVec<u8>  = bitvec![1, 0, 0, 0, 1, 1, 1, 1];
    //     assert!(!p.search(&b11));
    //     let b12: BitVec<u8>  = bitvec![0, 0, 0, 0, 1, 1, 1, 0];
    //     assert!(!p.search(&b12));
    //     let b13: BitVec<u8>  = bitvec![0, 0, 0, 0, 1, 0, 1, 1];
    //     assert!(!p.search(&b13));
    //     let b14: BitVec<u8>  = bitvec![0, 0, 0, 0, 0, 0];
    //     assert!(!p.search(&b14));
    // }

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

