// maelstrom
// Copyright (C) 2020 Raphael Robert
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see http://www.gnu.org/licenses/.

use crate::messages::*;
use std::cmp::Ordering;

pub fn log2(x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut k = 0;
    while (x >> k) > 0 {
        k += 1
    }
    k - 1
}

pub fn pow2(x: usize) -> usize {
    match x {
        0 => 1,
        _ => 2 << (x - 1),
    }
}

pub fn level(index: TreeIndex) -> usize {
    let x = index.as_usize();
    if (x & 0x01) == 0 {
        return 0;
    }
    let mut k = 0;
    while ((x >> k) & 0x01) == 1 {
        k += 1;
    }
    k
}

pub fn node_width(n: usize) -> usize {
    2 * (n - 1) + 1
}

pub fn root(size: RosterIndex) -> TreeIndex {
    let n = size.as_usize();
    let w = node_width(n);
    TreeIndex::from((1usize << log2(w)) - 1)
}

pub fn left(index: TreeIndex) -> TreeIndex {
    let x = index.as_usize();
    let k = level(TreeIndex::from(x));
    if k == 0 {
        return TreeIndex::from(x);
    }
    TreeIndex::from(x ^ (0x01 << (k - 1)))
}

pub fn right(index: TreeIndex, size: RosterIndex) -> TreeIndex {
    let x = index.as_usize();
    let n = size.as_usize();
    let k = level(TreeIndex::from(x));
    if k == 0 {
        return TreeIndex::from(x);
    }
    let mut r = x ^ (0x03 << (k - 1));
    while r >= node_width(n) {
        r = left(TreeIndex::from(r)).as_usize();
    }
    TreeIndex::from(r)
}

pub fn parent_step(x: usize) -> usize {
    let k = level(TreeIndex::from(x));
    let b = (x >> (k + 1)) & 0x01;
    (x | (1 << k)) ^ (b << (k + 1))
}

pub fn parent(index: TreeIndex, size: RosterIndex) -> TreeIndex {
    let x = index.as_usize();
    let n = size.as_usize();
    if index == root(size) {
        return index;
    }
    let mut p = parent_step(x);
    while p >= node_width(n) {
        p = parent_step(p)
    }
    TreeIndex::from(p)
}

pub fn sibling(index: TreeIndex, size: RosterIndex) -> TreeIndex {
    let p = parent(index, size);
    match index.cmp(&p) {
        Ordering::Less => right(p, size),
        Ordering::Greater => left(p),
        Ordering::Equal => p,
    }
}

// Ordered from leaf to root
// Includes neither leaf nor root
pub fn dirpath(index: TreeIndex, size: RosterIndex) -> Vec<TreeIndex> {
    let mut d = vec![];
    let mut p = parent(index, size);
    let r = root(size);
    while p != r {
        d.push(p);
        p = parent(p, size);
    }
    d
}

// Ordered from leaf to root
// Includes leaf and root
pub fn dirpath_long(index: TreeIndex, size: RosterIndex) -> Vec<TreeIndex> {
    let mut d = vec![index];
    let mut p = parent(index, size);
    let r = root(size);
    if index == r {
        return vec![p];
    }
    while p != r {
        d.push(p);
        p = parent(p, size);
    }
    d.push(r);
    d
}

// Ordered from leaf to root
// Includes root but not leaf
pub fn dirpath_root(index: TreeIndex, size: RosterIndex) -> Vec<TreeIndex> {
    let mut d = vec![];
    let mut p = parent(index, size);
    let r = root(size);
    while p != r {
        d.push(p);
        p = parent(p, size);
    }
    d.push(r);
    d
}

// Ordered from leaf to root
pub fn copath(index: TreeIndex, size: RosterIndex) -> Vec<TreeIndex> {
    let mut d = vec![index];
    d.append(&mut dirpath(index, size));
    d.iter().map(|&index| sibling(index, size)).collect()
}

pub fn common_ancestor(x: TreeIndex, y: TreeIndex) -> TreeIndex {
    let (mut xn, mut yn) = (x.as_usize(), y.as_usize());
    let mut k = 0;
    while xn != yn {
        xn >>= 1;
        yn >>= 1;
        k += 1;
    }
    TreeIndex::from((xn << k) + (1 << (k - 1)) - 1)
}


#[test]
fn verify_binary_test_vector_treemath() {
    use crate::codec::*;
    use crate::treemath;
    use crate::utils::*;

    use serde::{self, Deserialize, Serialize};
    pub use std::fs::File;
    pub use std::io::{BufReader, prelude::*};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[allow(non_snake_case)]
    struct Test {
        size: u32,
        root: String,
        left: String,
        right: String,
        parent: String,
        sibling: String
    }
    impl Test {
        fn from_file(file: &'static str) -> Self {
            let file = match File::open(file) {
                Ok(f) => f,
                Err(_) => panic!("Couldn't open file {}.", file),
            };
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(r) => r,
                Err(e) => {
                    println!("{:?}", e);
                    panic!("Error reading file.")
                },
            }
        }
        fn write_file(&self, file: &'static str) {
            let mut file = match File::create(file) {
                Ok(f) => f,
                Err(_) => panic!("Couldn't open file {}.", file),
            };
            let json = match serde_json::to_string_pretty(&self) {
                Ok(j) => j,
                Err(_) => panic!("Couldn't serialize this object."),
            };
            match file.write_all(&json.into_bytes()) {
                Ok(_) => (),
                Err(_) => panic!("Error writing to file."),
            }
        }
    }


    let mut file = File::open("test_vectors/tree_math.bin").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let cursor = &mut Cursor::new(&buffer);

    let tree_size = RosterIndex::from(u32::decode(cursor).unwrap());

    let root: Vec<u32> = decode_vec(VecSize::VecU32, cursor).unwrap();
    let root_s = bytes_to_hex(&u32_to_bytes(&root));
    let left: Vec<u32> = decode_vec(VecSize::VecU32, cursor).unwrap();
    let left_s = bytes_to_hex(&u32_to_bytes(&left));
    let right: Vec<u32> = decode_vec(VecSize::VecU32, cursor).unwrap();
    let right_s = bytes_to_hex(&u32_to_bytes(&right));
    let parent: Vec<u32> = decode_vec(VecSize::VecU32, cursor).unwrap();
    let parent_s = bytes_to_hex(&u32_to_bytes(&parent));
    let sibling: Vec<u32> = decode_vec(VecSize::VecU32, cursor).unwrap();
    let sibling_s = bytes_to_hex(&u32_to_bytes(&sibling));
    let t = Test {
        size: tree_size.as_u32(),
        root: root_s,
        left: left_s,
        right: right_s,
        parent: parent_s,
        sibling: sibling_s
    };
    t.write_file("test_vectors/tree_math.json");

    let file_json = Test::from_file("test_vectors/tree_math.json");
    let tree_size_json = RosterIndex::from(file_json.size);
    let root_json = bytes_to_u32(&hex_to_bytes(&file_json.root));
    let left_json = bytes_to_u32(&hex_to_bytes(&file_json.left));
    let right_json = bytes_to_u32(&hex_to_bytes(&file_json.right));
    let sibling_json = bytes_to_u32(&hex_to_bytes(&file_json.sibling));

    assert_eq!(tree_size_json, tree_size);
    assert_eq!(root_json, root);
    assert_eq!(left_json, left);
    assert_eq!(right_json, right);
    assert_eq!(sibling_json, sibling);

    for (i, r) in root.iter().enumerate() {
        assert_eq!(
            TreeIndex::from(*r),
            treemath::root(RosterIndex::from(i + 1))
        );
    }
    for (i, l) in left.iter().enumerate() {
        assert_eq!(TreeIndex::from(*l), treemath::left(TreeIndex::from(i)));
    }
    for (i, r) in right.iter().enumerate() {
        assert_eq!(
            TreeIndex::from(*r),
            treemath::right(TreeIndex::from(i), tree_size)
        );
    }
    for (i, p) in parent.iter().enumerate() {
        assert_eq!(
            TreeIndex::from(*p),
            treemath::parent(TreeIndex::from(i), tree_size)
        );
    }
    for (i, s) in sibling.iter().enumerate() {
        assert_eq!(
            TreeIndex::from(*s),
            treemath::sibling(TreeIndex::from(i), tree_size)
        );
    }
    assert_eq!(cursor.has_more(), false);
}
