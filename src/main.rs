use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
enum HuffmanTree<T> {
    Leaf(T),
    Fork(Box<HuffmanTree<T>>, Box<HuffmanTree<T>>),
}
// From [T] | [T * F] where T: Eq + probably Hash, F: Ord + maybe Num
impl<T> HuffmanTree<T> {}

#[derive(Debug)]
struct FrequencyPair<T, F: Ord> {
    symbol: T,
    freq: F,
}

impl<T, F> Ord for FrequencyPair<T, F>
where
    F: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq).reverse()
    }
}

impl<T, F> PartialOrd for FrequencyPair<T, F>
where
    F: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, F> PartialEq for FrequencyPair<T, F>
where
    F: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl<T, F> Eq for FrequencyPair<T, F> where F: Ord {}

fn main() {
    let test_string = "Huffman for fun";
    let mut frequency_map = HashMap::new();

    for c in test_string.chars() {
        frequency_map
            .entry(c)
            .and_modify(|count: &mut u32| *count = count.saturating_add(1))
            .or_insert(1);
    }
    println!("{:?}", frequency_map);

    let mut p_queue: BinaryHeap<_> = frequency_map
        .into_iter()
        .map(|(symbol, freq)| FrequencyPair { symbol, freq })
        .collect();

    while let Some(i) = p_queue.pop() {
        println!("{:?}", i);
    }

    let ha = HuffmanTree::Leaf('a');
    let hb = HuffmanTree::Leaf('b');

    let h = HuffmanTree::Fork(Box::new(ha), Box::new(hb));
    println!("{:?}", h);
}
