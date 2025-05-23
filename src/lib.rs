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
    fn fork(a: Self, b: Self) -> Self {
        Self::Fork(Box::new(a), Box::new(b))
    }
    /// Generate a `HuffmanTree` from a string slice by counting the occurence of each character. Returns None if string is empty.
    pub fn tree(string: &str) -> Option<Self> {
        let mut frequency_map = HashMap::new();

        for c in string.chars() {
            frequency_map
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut p_queue: BinaryHeap<_> = frequency_map
            .into_iter()
            .map(|(symbol, freq)| FrequencyPair(freq, Self::Leaf(symbol)))
            .collect();

        while let Some(FrequencyPair(fa, a)) = p_queue.pop() {
            if let Some(FrequencyPair(fb, b)) = p_queue.pop() {
                p_queue.push(FrequencyPair(
                    fa.saturating_add(fb), // If for some reason the integer overflows it's more useful to limit than wrap
                    Self::fork(a, b),
                ));
            } else {
                return match a {
                    Self::Leaf(_) => Some(Self::fork(a, Self::Leaf(char::default()))), // Trees for a single character still need a root node
                    Self::Fork(..) => Some(a),
                };
            }
        }
        None
    }
    /// Convenience function for encoding a single character
    fn enc_char(&self, ch: char) -> BitVec {
        fn aux(tree: &HuffmanTree, ch: char, bv: &mut BitVec) -> bool {
            match tree {
                HuffmanTree::Leaf(c) => *c == ch,
                HuffmanTree::Fork(left, right) => {
                    if aux(left, ch, bv) {
                        bv.push(false);
                        return true;
                    } else if aux(right, ch, bv) {
                        bv.push(true);
                        return true;
                    }
                    false
                }
            }
        }
        let mut bv = BitVec::new();
        aux(self, ch, &mut bv);
        bv.reverse();
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
