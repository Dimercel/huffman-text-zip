use std::collections::HashMap;
use std::cmp::Reverse;
use std::env;

use easy_tree::Tree;

type FreqMap = HashMap<char, u32>;

#[derive(Debug, Clone)]
enum Bit {One, Zero}

type Code = Vec<Bit>;
type CodeMap = HashMap<char, Code>;
type CharWeight = (u32, Option<char>);


fn display_code(code: &Code) -> String {
    code.into_iter().map(|x| match x {Bit::Zero => '0', Bit::One => '1'}).collect()
}

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
        let mut left_node: Option<CharWeight> = None;
        let mut right_node: Option<CharWeight> = None;

        match (pair.get(0), pair.get(1)) {
            (Some(inx1), Some(inx2)) => {
                left_node = Some(*prefix_tree.get(*inx1).unwrap());
                right_node = Some(*prefix_tree.get(*inx2).unwrap());
            },
            (Some(inx), _) => {left_node = Some(*prefix_tree.get(*inx).unwrap())},
            (_, Some(inx)) => {right_node = Some(*prefix_tree.get(*inx).unwrap())},
            _ => {}
        };

        match left_node {
            Some((_, Some(ch))) => {
                let mut full_code = code.clone();
                full_code.push(Bit::One);
                codes_map.insert(ch, full_code);

                code.push(Bit::Zero);
            },
            Some((_, None)) => {node = pair[0]},
            _ => {}
        };

        match right_node {
            Some((_, Some(ch))) => {
                code.push(Bit::Zero);
                codes_map.insert(ch, code.clone());

                code.push(Bit::Zero);
            },
            Some((_, None)) => {node = pair[1]; },
            _ => {}
        };

        if !pair.contains(&node) {
            break;
        }

    }

    codes_map
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let text = &args[1];
    println!("Original: \"{}\"", text);
    println!("Original size: {} bytes", text.len());

    let prefix_tree = build_tree(&count_freq(text));
    let codes = build_codes(&prefix_tree);
    let mut encoded: Code = vec![];

    for ch in text.chars() {
        match codes.get(&ch) {
            Some(bits) => encoded.extend_from_slice(bits),
            _ => {}
        };
    }

    println!("Encoded size: {} bytes", encoded.len() / 8);
    println!("Encoded: \"{}\"", display_code(&encoded));
}
