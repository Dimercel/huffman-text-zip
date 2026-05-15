use std::collections::HashMap;
use std::cmp::Reverse;

use easy_tree::Tree;

type FreqMap = HashMap<char, u32>;

#[derive(Debug)]
enum Bit {One, Zero}

type Code = Vec<Bit>;
type CodeMap = HashMap<char, Code>;
type CharWeight = (u32, Option<char>);


/// Подсчитывает кол-во вхождений каждого уникального
/// символа в строке и возвращает значение
fn count_freq(text: &str) -> FreqMap {
    let mut res = FreqMap::new();

    for c in text.chars() {
        *res.entry(c).or_insert(0) += 1;
    }

    res
}

// Строит двоичное дерево, листьями которого являются конкретные символы.
// По этому дереву можно будет построить уникальный префикс для любого символа.
fn build_tree(freq: &FreqMap) -> Tree<CharWeight> {
    if freq.is_empty() {
        return Tree::new();
    }

    let mut node_list: Vec<(&char, &u32)> = freq.into_iter().collect();
    node_list.sort_by_key(|x| Reverse(x.1));

    let mut tree = Tree::new();
    let mut rest_weight: u32 = node_list.iter().map(|x| x.1).sum();
    let last_node = node_list.pop();
    let mut sub_tree = tree.add_node((rest_weight, None));
    for (ch, cnt) in node_list {
        tree.add_child(sub_tree, (*cnt, Some(*ch)));
        rest_weight -= *cnt;
        if rest_weight != 0 {
            sub_tree = tree.add_child(sub_tree, (rest_weight, None));
        }
    };

    if let Some(x) = last_node {
        tree.add_child(sub_tree, (*x.1, Some(*x.0)));
    };

    tree
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", Bit::One);
    println!("{:?}", count_freq("aaabbc"));
    let mut freq_list: Vec<(char, u32)> = count_freq("aaabbcdddd").into_iter().collect();
    freq_list.sort_by_key(|x| x.1);

    println!("{:?}", freq_list);
    let mut prefix_tree = build_tree(&count_freq("aaabbc"));

    for (idx, data) in prefix_tree.iter_mut() {
        println!("{:?}", (idx, data));
    };
}
