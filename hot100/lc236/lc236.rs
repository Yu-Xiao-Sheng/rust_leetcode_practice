#!/usr/bin/env rust-script

//! 236. 二叉树的最近公共祖先（Hot100）
//! 递归后序：时间 O(n)，空间 O(h)

use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Node = Rc<RefCell<TreeNode>>;
type Link = Option<Node>;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn new(val: i32) -> Node {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

fn lowest_common_ancestor(root: Link, p: Node, q: Node) -> Link {
    fn dfs(cur: &Link, p: &Node, q: &Node) -> Link {
        match cur {
            None => None,
            Some(node) => {
                if Rc::ptr_eq(node, p) || Rc::ptr_eq(node, q) {
                    return cur.clone();
                }

                let (left_child, right_child) = {
                    let borrowed = node.borrow();
                    (borrowed.left.clone(), borrowed.right.clone())
                };

                let left_hit = dfs(&left_child, p, q);
                let right_hit = dfs(&right_child, p, q);

                match (left_hit, right_hit) {
                    (Some(_), Some(_)) => cur.clone(), // p、q 分别在左右子树
                    (Some(node), None) | (None, Some(node)) => Some(node),
                    (None, None) => None,
                }
            }
        }
    }

    dfs(&root, &p, &q)
}

fn build_tree_with_map(data: &[Option<i32>]) -> (Link, HashMap<i32, Node>) {
    if data.is_empty() || data[0].is_none() {
        return (None, HashMap::new());
    }

    let mut nodes: Vec<Link> = Vec::with_capacity(data.len());
    let mut map = HashMap::new();

    for &val in data {
        if let Some(v) = val {
            let node = TreeNode::new(v);
            map.insert(v, node.clone());
            nodes.push(Some(node));
        } else {
            nodes.push(None);
        }
    }

    for i in 0..data.len() {
        if let Some(node) = nodes[i].clone() {
            let left_idx = 2 * i + 1;
            let right_idx = 2 * i + 2;
            let mut borrowed = node.borrow_mut();

            if left_idx < data.len() {
                borrowed.left = nodes[left_idx].clone();
            }
            if right_idx < data.len() {
                borrowed.right = nodes[right_idx].clone();
            }
        }
    }

    (nodes[0].clone(), map)
}

fn run_case(data: &[Option<i32>], p: i32, q: i32, label: &str) {
    let (root, map) = build_tree_with_map(data);
    let (p_node, q_node) = match (map.get(&p), map.get(&q)) {
        (Some(pn), Some(qn)) => (pn.clone(), qn.clone()),
        _ => {
            println!("案例 {label}: 输入的 p 或 q 不在树中");
            return;
        }
    };

    let lca = lowest_common_ancestor(root, p_node, q_node);
    println!("案例 {label}: p = {p}, q = {q}");
    match lca {
        Some(node) => println!("最近公共祖先: {}", node.borrow().val),
        None => println!("未找到公共祖先"),
    }
    println!();
}

fn main() {
    let sample1 = vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ];
    run_case(&sample1, 5, 1, "示例一");
    run_case(&sample1, 5, 4, "示例二");

    let sample2 = vec![Some(1), Some(2)];
    run_case(&sample2, 1, 2, "示例三");
}
