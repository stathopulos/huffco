use display_tree::println_tree;
use huffco::HuffmanTree;

fn main() {
    let s = "Huffman for fun";
    let t = HuffmanTree::tree(s).unwrap();
    let e = t.enc(s);

    println_tree!(t);
    println!("string: \"{s}\"");
    println!("encoded: {e:b}",);
    println!("decoded: \"{}\"", t.dec(&e));

    println!("Number of bits for string: {}", 8 * s.len());
    println!("Number of bits compressed: {}", e.len());
}
