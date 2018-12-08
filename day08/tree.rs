use std::fs;

#[derive(Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

fn build_tree(it: &mut std::slice::Iter<u32>) -> Option<Node> {
    let number_of_children = match it.next() {
        Some(&number) => number,
        None => return None,
    };
    let metadata_size = it.next().unwrap();

    let mut node = Node::default();
    for _ in 0..number_of_children {
        node.children.extend(build_tree(it));
    }

    for _ in 0..*metadata_size {
        node.metadata.push(*it.next().unwrap());
    }

    Some(node)
}

fn calculate_metadata_sum(node: &Node) -> u32 {
    node.metadata.iter().sum::<u32>() + node
        .children
        .iter()
        .map(|c| calculate_metadata_sum(c))
        .sum::<u32>()
}

fn calculate_node_value(node: &Node) -> u32 {
    if node.children.is_empty() {
        node.metadata.iter().sum()
    } else {
        node.metadata
            .iter()
            .filter(|&v| v - 1 < node.children.len() as u32)
            .map(|index| calculate_node_value(&node.children[(index - 1) as usize]))
            .sum()
    }
}

fn main() {
    let mut tree = Vec::new();
    for number in fs::read_to_string("input").unwrap().split_whitespace() {
        tree.push(number.parse::<u32>().unwrap());
    }

    let tree = build_tree(&mut tree.iter()).unwrap();
    println!("{}", calculate_metadata_sum(&tree));
    println!("{}", calculate_node_value(&tree));
}
