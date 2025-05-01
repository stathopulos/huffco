pub use bitvec::vec::BitVec;
use display_tree::DisplayTree;
use std::collections::{BinaryHeap, HashMap};

#[derive(Ord, PartialOrd, PartialEq, Eq, DisplayTree)]
pub enum HuffmanTree {
    Leaf(char),
    Fork(#[tree] Box<HuffmanTree>, #[tree] Box<HuffmanTree>),
}

impl HuffmanTree {
    /// Convenience function for creating a `Fork` from two nodes
    fn fork(a: HuffmanTree, b: HuffmanTree) -> HuffmanTree {
        HuffmanTree::Fork(Box::new(a), Box::new(b))
    }
    /// Generate a `HuffmanTree` from a string slice by counting the occurence of each character
    pub fn tree(string: &str) -> Option<HuffmanTree> {
        let mut frequency_map = HashMap::new();

        for c in string.chars() {
            frequency_map
                .entry(c)
                .and_modify(|count: &mut u32| *count = count.saturating_add(1)) // If for some reason the integer overflows it's more useful to limit than wrap
                .or_insert(1);
        }

        let mut p_queue: BinaryHeap<_> = frequency_map
            .into_iter()
            .map(|(symbol, freq)| FrequencyPair(freq, HuffmanTree::Leaf(symbol)))
            .collect();

        while let Some(FrequencyPair(fa, a)) = p_queue.pop() {
            if let Some(FrequencyPair(fb, b)) = p_queue.pop() {
                p_queue.push(FrequencyPair(
                    fa.saturating_add(fb), // If for some reason the integer overflows it's more useful to limit than wrap
                    HuffmanTree::fork(a, b),
                ));
            } else {
                return Some(a);
            }
        }
        None
    }
    /// Convenience function for encoding a single character
    fn enc_char(&self, ch: char) -> BitVec {
        let mut stack = Vec::new();
        let bv = BitVec::new();
        stack.push((self, bv.clone()));
        while let Some((node, result)) = stack.pop() {
            match node {
                Self::Leaf(c) => {
                    if *c == ch {
                        return result;
                    }
                }
                Self::Fork(a, b) => {
                    let mut left = result.clone();
                    left.push(false);
                    stack.push((a, left));

                    let mut right = result.clone();
                    right.push(true);
                    stack.push((b, right));
                }
            }
        }
        bv
    }
    /// Use tree to encode a string to a `BitVec`
    pub fn enc(&self, string: &str) -> BitVec {
        string.chars().flat_map(|c| self.enc_char(c)).collect()
    }
    /// Convenience function for traversing the tree based on a single bit. 0 for left, 1 for right.
    fn desc_tree(&self, bit: bool) -> &Self {
        match self {
            Self::Fork(a, b) => {
                if bit {
                    b.as_ref()
                } else {
                    a.as_ref()
                }
            }
            Self::Leaf(_) => self,
        }
    }
    /// Decode a `BitVec` encoded with the tree to a `String`
    pub fn dec(&self, bv: &BitVec) -> String {
        let mut cv = String::new();
        let mut node = self;
        for i in bv {
            match node.desc_tree(*i) {
                Self::Leaf(c) => {
                    cv.push(*c);
                    node = self;
                }
                fork @ Self::Fork(..) => node = fork,
            }
        }
        cv
    }
}

#[derive(PartialEq, Eq)]
struct FrequencyPair(u32, HuffmanTree);

/// Order by frequency in reverse order for min-queue, then by the tree's natual order for reproducibility
impl Ord for FrequencyPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse().then(self.1.cmp(&other.1))
    }
}

impl PartialOrd for FrequencyPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
