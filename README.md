# Huffman
A not terribly efficient implementation of a huffman tree created for my Discrete Structures course. This program takes a single string as an argument, uses it to build a huffman tree by counting the number of occurrences of each character, encodes the string as a binary value, back decodes the string, and prints it all to the console along with the number of bits for the compressed and uncompressed version

## Usage
Call the program with a single string as an argument  
`huffco <STRING>`

```console
$ huffco "huffman"
Fork
├── Fork
│   ├── Leaf
│   │   └── a
│   └── Fork
│       ├── Leaf
│       │   └── u
│       └── Leaf
│           └── n
└── Fork
    ├── Fork
    │   ├── Leaf
    │   │   └── m
    │   └── Leaf
    │       └── h
    └── Leaf
        └── f
string: "huffman"
encoded: [101010111110000011]
└── back decoded: "huffman"
Number of bits in 8-bit ASCII: 56
Number of bits compressed: 18
```

## Benchmarking
In my testing, the program runs correctly and in a reasonable enough time for inputs smaller than the max argument size (1048576 chars on my system). Some testing I did with the largest plain-text files under 1MB I could find on my system:
- an 852kb comma-separated log file of numeric values with a header:  
`940.2 ms ±   5.1 ms`
- a random 899kb js file I pulled from some local Excel system files:  
`2.243 s ±  0.026 s`

## Dependencies
This project uses only two small dependencies, neither of which is entirely necessary, but they're both nice to have
- `bitvec` provides a growable collection of bits as a drop-in alternative to the standard library's `Vec<bool>`
- `display-tree` provides a convenient derive macro and the `print_tree` function for printing tree structures to stdout with minimal effort
