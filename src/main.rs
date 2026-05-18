use std::collections::HashMap;
use std::cmp::Reverse;

use easy_tree::Tree;

type FreqMap = HashMap<char, u32>;

#[derive(Debug, Clone)]
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

/// Строит двоичное дерево, листьями которого являются конкретные символы.
/// По этому дереву можно будет построить словарь уникальных префиксов для всех символов
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

/// Выстраивает уникальный двоичный идентификатор для каждого символа в дереве
fn build_codes(prefix_tree: &Tree<CharWeight>) -> CodeMap {
    let mut node = 0;
    let mut code: Code = Vec::new();
    let mut codes_map = CodeMap::new();

    loop {
        let pair = prefix_tree.children(node);
        println!("{:?}", pair);
        let left_node = prefix_tree.get(pair[0]).unwrap();
        let right_node = prefix_tree.get(pair[1]).unwrap();

        match left_node {
            (_, Some(x)) => {
                let mut full_code = code.clone();
                full_code.push(Bit::One);
                codes_map.insert(*x, full_code);
            },
            _ => {node = pair[0]; }
        };

        match right_node {
            (_, Some(x)) => {
                let mut full_code = code.clone();
                full_code.push(Bit::Zero);
                codes_map.insert(*x, full_code);
            },
            _ => {node = pair[1]; }
        };

        if left_node.1.is_some() {
            code.push(Bit::One)
        } else {
            code.push(Bit::Zero)
        };


        if node != pair[0] && node != pair[1] {
            break;
        }

    }

    codes_map
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

    let codes = build_codes(&prefix_tree);
}
