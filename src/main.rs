use std::collections::HashMap;

use easy_tree::Tree;

type FreqMap = HashMap<char, u32>;

#[derive(Debug)]
enum Bit {One, Zero}

type Code = Vec<Bit>;
type CodeMap = HashMap<char, Code>;
type CharWeight = (u32, char);

fn build_tree(freq: &FreqMap) -> Tree<CharWeight> {
    Tree::new()
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", Bit::One);

}
