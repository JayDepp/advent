use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cell::Cell;

use scan_rules::scanner::Word;

// #[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    children: Cell<Vec<&'a Node<'a>>>,
}

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

pub fn day7() -> Result<(), Box<Error>> {
    let file = File::open("day7.txt")?;
    
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let programs: Vec<_> = lines
        .iter()
        .map(|line| parse_program(&line))
        .collect();

    let nodes: Vec<_> = programs
        .iter()
        .map(|program|
            Node {
                name: program.name,
                weight: program.weight,
                children: Cell::new(vec![])
            }
        )
        .collect();

    let mut stack: HashMap<&str, &Node> = HashMap::new();
    let mut used: HashMap<&str, &Node> = HashMap::new();

    for node in &nodes {
        stack.insert(node.name, &node);
    }

    for program in programs {
        let Program { name, weight: _, children } = program;
        if !children.is_empty() {
            let node = if let Some(node) = stack.get(name) {
                node
            } else {
                used[name]
            };
            let children = children
                .iter()
                .map(|child| {
                    let child_node = stack.remove(child).unwrap();
                    used.insert(child, child_node);
                    child_node
                })
                .collect();
            node.children.set(children);
        }
    }

    for (name, _) in stack {
        println!("{}", name);
    }

    Ok(())
}

fn parse_program(text: &str) -> Program {
    let_scan!(text; (
        let name: Word<&str>,
        "(",
        let weight: u32,
        ")",
        [ "->" ]?,
        [ let children: Word<&str> ],*
    ));

    Program { name, weight, children }
}