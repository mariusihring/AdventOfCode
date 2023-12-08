use indexmap::IndexMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Solution;

impl Solution {
    pub fn prepare() -> NodeDirections {
        let mut directions: Vec<usize> = Vec::new();
        let mut lines = include_str!("./inputs/input").lines();
        let instructions = lines.next().unwrap();
        let mut nodes: IndexMap<String, Vec<String>> = IndexMap::new();
        instructions.chars().for_each(|c| {
            if c == 'R' {
                directions.push(1)
            } else {
                directions.push(0)
            }
        });

        lines.for_each(|line| {
            if !line.is_empty() {
                let mut line = line.split(" = ");
                let name = line.next().unwrap().trim_end().to_owned();
                let mut children = line.next().unwrap().trim_start().to_owned();
                let first = children.replace("(", "");
                let second = first.replace(")", "");
                let cleaned_up = second.split(",");
                let temp = cleaned_up
                    .map(|child| child.trim().to_owned())
                    .collect::<Vec<String>>();

                nodes.insert(name, temp);
            }
        });
        NodeDirections { directions, nodes }
    }

    pub fn solve_first() {
        let node_directions = Self::prepare();
        let mut current_node = "AAA";
        let mut steps = 0;
        loop {
            if current_node == "ZZZ" {
                break;
            }
            for dir in node_directions.directions.iter() {
                current_node = &(*node_directions.nodes.get(current_node).unwrap())[*dir];
                println!("{current_node}");
                steps += 1;
                if current_node == "ZZZ" {
                    break;
                }
            }
        }

        println!("part 1 : {}", steps)
    }

    pub fn solve_second() {
        let node_directions = Self::prepare();
        let mut starting_nodes: Vec<String> = node_directions
            .nodes
            .iter()
            .filter(|(name, children)| name.ends_with("A"))
            .map(|(&ref name, _)| name.to_owned())
            .collect();
        let mut steps = 0;
        'test: loop {
            for dir in node_directions.directions.iter() {
                for node in starting_nodes.iter_mut() {
                    *node = (*node_directions.nodes.get(node).unwrap())[*dir].clone();
                }
                steps += 1;
                println!("{:?}", starting_nodes);
                if starting_nodes.iter().all(|x| x.ends_with("Z")) {
                    break 'test;
                }
            }
        }
        println!("part 2 : {:?}", steps)
    }
}

fn main() {
    // Solution::solve_first()
    Solution::solve_second()
}

struct NodeDirections {
    directions: Vec<usize>,
    nodes: IndexMap<String, Vec<String>>,
}
