use huffco::HuffmanTree;
use std::env;

use display_tree::println_tree;

fn parse_args(args: &mut env::Args) -> Result<String, &'static str> {
    match (args.next(), args.next(), args.next()) {
        (_, _, Some(_)) => {
            Err("Too many arguments! This program only accepts a single string as input!")
        }
        (_, None, _) => {
            Err("No arguments provided! This program requires a single string as input!")
        }
        (_, Some(s), None) => Ok(s),
    }
}

fn main() -> Result<(), &'static str> {
    let input_string = parse_args(&mut env::args())?;
    let tree = HuffmanTree::tree(&input_string)
        .ok_or("Input string is empty! Cannot create a huffman tree from an empty string!")?;
    let encoded_bits = tree.enc(&input_string);

    println_tree!(tree);
    println!("string: \"{input_string}\"");
    println!("encoded: {encoded_bits:b}",);
    println!("└── back decoded: \"{}\"", tree.dec(&encoded_bits));

    println!("Number of bits in 8-bit ASCII: {}", 8 * input_string.len());
    println!("Number of bits compressed: {}", encoded_bits.len());
    Ok(())
}
