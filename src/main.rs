use bitvec::vec::BitVec;
use display_tree::{DisplayTree, println_tree};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, DisplayTree)]
enum HuffmanTree {
    Leaf(char),
    Fork(#[tree] Box<HuffmanTree>, #[tree] Box<HuffmanTree>),
}

impl HuffmanTree {
    /// Convenience function for creating a fork from two nodes
    fn fork(a: HuffmanTree, b: HuffmanTree) -> HuffmanTree {
        HuffmanTree::Fork(Box::new(a), Box::new(b))
    }
    /// Generate a Huffman tree from a string slice
    fn tree(string: &str) -> Option<HuffmanTree> {
        let mut frequency_map = HashMap::new();

        for c in string.chars() {
            frequency_map
                .entry(c)
                .and_modify(|count: &mut u32| *count = count.saturating_add(1))
                .or_insert(1);
        }

        let mut p_queue: BinaryHeap<_> = frequency_map
            .into_iter()
            .map(|(symbol, freq)| FrequencyPair(freq, HuffmanTree::Leaf(symbol)))
            .collect();

        while let Some(FrequencyPair(fa, a)) = p_queue.pop() {
            if let Some(FrequencyPair(fb, b)) = p_queue.pop() {
                p_queue.push(FrequencyPair(
                    fa.saturating_add(fb),
                    HuffmanTree::fork(a, b),
                ));
            } else {
                return Some(a);
            }
        }
        None
    }
    fn enc_char(&self, ch: char) -> BitVec {
        // let bv = BitVec::new();
        let mut vv = Vec::new();
        let bv = BitVec::new();
        vv.push((self, bv.clone()));
        while let Some((node, result)) = vv.pop() {
            match node {
                Self::Leaf(c) => {
                    if *c == ch {
                        return result;
                    }
                }
                Self::Fork(a, b) => {
                    let mut left = result.clone();
                    left.push(false);
                    vv.push((a, left));

                    let mut right = result.clone();
                    right.push(true);
                    vv.push((b, right));
                }
            }
        }
        bv
    }
    fn enc(&self, string: &str) -> BitVec {
        string.chars().flat_map(|c| self.enc_char(c)).collect()
    }
}

#[derive(PartialEq, Eq)]
struct FrequencyPair(u32, HuffmanTree);

/// Order by frequency in reverse order for min-queue, then by character for reproducibility
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

fn main() {
    let s = "Huffman for fun";
    let t = HuffmanTree::tree(s).unwrap();

    println_tree!(t);
    println!("string: \"{s}\"");
    println!("encoded: {}", t.enc(s));
}
