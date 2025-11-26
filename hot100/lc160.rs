#!/usr/bin/env rust-script

//! 160. 相交链表（Hot100）
//! 时间 O(m+n)，空间 O(1)
use std::rc::Rc;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<ListNode>>,
}

impl ListNode {
    fn new(val: i32, next: Option<Rc<ListNode>>) -> Rc<ListNode> {
        Rc::new(ListNode { val, next })
    }
}

/// 经典双指针：两指针分别从两链表出发，走完本链表后切换到另一条，
/// 最多走 m+n 步就会在相交点或同时到达 None。
fn get_intersection_node(
    head_a: Option<Rc<ListNode>>,
    head_b: Option<Rc<ListNode>>,
) -> Option<Rc<ListNode>> {
    let mut pa = head_a.clone();
    let mut pb = head_b.clone();

    while !ptr_eq_option(&pa, &pb) {
        pa = match pa {
            Some(node) => node.next.clone(),
            None => head_b.clone(),
        };
        pb = match pb {
            Some(node) => node.next.clone(),
            None => head_a.clone(),
        };
    }

    pa
}

fn ptr_eq_option(a: &Option<Rc<ListNode>>, b: &Option<Rc<ListNode>>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        (None, None) => true,
        _ => false,
    }
}

fn build_list(vals: &[i32], tail: Option<Rc<ListNode>>) -> Option<Rc<ListNode>> {
    let mut head = tail;
    for &v in vals.iter().rev() {
        head = Some(ListNode::new(v, head));
    }
    head
}

/// 构造两个带公共尾巴的链表，方便演示相交。
fn build_intersected_lists(
    prefix_a: &[i32],
    prefix_b: &[i32],
    common: &[i32],
) -> (Option<Rc<ListNode>>, Option<Rc<ListNode>>, Option<Rc<ListNode>>) {
    let common_head = build_list(common, None);
    let head_a = build_list(prefix_a, common_head.clone());
    let head_b = build_list(prefix_b, common_head.clone());
    (head_a, head_b, common_head)
}

fn print_list(head: &Option<Rc<ListNode>>) {
    let mut cur = head.clone();
    let mut vals = Vec::new();
    while let Some(node) = cur {
        vals.push(node.val);
        cur = node.next.clone();
    }
    println!("{vals:?}");
}

fn main() {
    // 示例 1：有交点
    let (head_a, head_b, common) = build_intersected_lists(&[4, 1], &[5, 6, 1], &[8, 4, 5]);
    println!("List A:");
    print_list(&head_a);
    println!("List B:");
    print_list(&head_b);

    let inter = get_intersection_node(head_a.clone(), head_b.clone());
    match &inter {
        Some(node) => println!("交点值: {}", node.val),
        None => println!("无交点"),
    }
    // 验证确实指向同一节点对象
    println!(
        "与 common 头节点同指针？ {}",
        ptr_eq_option(&inter, &common)
    );
    println!();

    // 示例 2：无交点
    let (head_a, head_b, _) = build_intersected_lists(&[2, 6, 4], &[1, 5], &[]);
    println!("List A:");
    print_list(&head_a);
    println!("List B:");
    print_list(&head_b);

    let inter = get_intersection_node(head_a, head_b);
    match inter {
        Some(node) => println!("交点值: {}", node.val),
        None => println!("无交点"),
    }
}
