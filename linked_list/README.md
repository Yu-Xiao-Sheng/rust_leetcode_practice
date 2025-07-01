# Rust 链表题目指导

## 🔗 目录
- [链表节点定义](#链表节点定义)
- [基本操作模板](#基本操作模板)
- [常见题型与技巧](#常见题型与技巧)
- [经典题目实现](#经典题目实现)
- [调试技巧](#调试技巧)

---

## 链表节点定义

### 单链表节点
```rust
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
```

### 双向链表节点
```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct DoublyListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<DoublyListNode>>>,
    pub prev: Option<Rc<RefCell<DoublyListNode>>>,
}

impl DoublyListNode {
    fn new(val: i32) -> Self {
        DoublyListNode {
            val,
            next: None,
            prev: None,
        }
    }
}
```

---

## 基本操作模板

### 1. 链表遍历
```rust
// 不可变遍历
fn traverse_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    while let Some(node) = current {
        println!("当前节点值: {}", node.val);
        current = &node.next;
    }
}

// 可变遍历
fn traverse_list_mut(mut head: Option<Box<ListNode>>) {
    while let Some(mut node) = head {
        println!("当前节点值: {}", node.val);
        head = node.next.take();
    }
}
```

### 2. 链表创建
```rust
// 从数组创建链表
fn create_list_from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    
    for val in vals {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    dummy.next
}

// 创建带环链表（用于测试）
fn create_cycle_list(vals: Vec<i32>, pos: i32) -> Option<Box<ListNode>> {
    if vals.is_empty() {
        return None;
    }
    
    let mut nodes: Vec<Box<ListNode>> = vals.into_iter()
        .map(|val| Box::new(ListNode::new(val)))
        .collect();
    
    // 连接节点
    for i in 0..nodes.len() - 1 {
        let next_node = nodes.swap_remove(i + 1);
        nodes[i].next = Some(next_node);
    }
    
    // 如果需要创建环
    if pos >= 0 && pos < nodes.len() as i32 {
        // 注意：在实际LeetCode中，环的创建需要用Rc<RefCell<>>
        // 这里仅作为概念演示
    }
    
    Some(nodes.swap_remove(0))
}
```

### 3. 链表长度计算
```rust
fn get_length(head: &Option<Box<ListNode>>) -> usize {
    let mut count = 0;
    let mut current = head;
    
    while let Some(node) = current {
        count += 1;
        current = &node.next;
    }
    
    count
}
```

---

## 常见题型与技巧

### 1. 双指针技巧

#### 快慢指针（Floyd判环算法）
```rust
// 检测链表是否有环
fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }
    
    let mut slow = &head;
    let mut fast = &head;
    
    loop {
        // 慢指针每次走一步
        if let Some(slow_node) = slow {
            slow = &slow_node.next;
        } else {
            return false;
        }
        
        // 快指针每次走两步
        if let Some(fast_node) = fast {
            if let Some(next_node) = &fast_node.next {
                fast = &next_node.next;
            } else {
                return false;
            }
        } else {
            return false;
        }
        
        // 检查是否相遇（这里需要指针比较，实际中可能需要其他方法）
        if std::ptr::eq(slow.as_ref().unwrap().as_ref(), 
                       fast.as_ref().unwrap().as_ref()) {
            return true;
        }
    }
}

// 找到链表的中点
fn find_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    
    slow.clone()
}
```

#### 相距k步的双指针
```rust
// 找到链表的倒数第k个节点
fn find_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut first = &head;
    let mut second = &head;
    
    // 第一个指针先走k步
    for _ in 0..k {
        if let Some(node) = first {
            first = &node.next;
        } else {
            return None; // k超出链表长度
        }
    }
    
    // 两个指针同时移动
    while first.is_some() {
        first = &first.as_ref().unwrap().next;
        second = &second.as_ref().unwrap().next;
    }
    
    second.clone()
}
```

### 2. 虚拟头节点技巧
```rust
// 删除链表中的节点
fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut current = &mut dummy;
    
    while current.next.is_some() {
        if current.next.as_ref().unwrap().val == val {
            let next_next = current.next.as_mut().unwrap().next.take();
            current.next = next_next;
            break;
        } else {
            current = current.next.as_mut().unwrap();
        }
    }
    
    dummy.next
}
```

### 3. 链表反转
```rust
// 反转整个链表
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;
    
    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    
    prev
}

// 反转链表的前n个节点
fn reverse_n(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if n == 1 {
        return head;
    }
    
    // 递归反转前n-1个节点
    if let Some(mut head_node) = head {
        let new_head = reverse_n(head_node.next.take(), n - 1);
        
        // 反转当前节点
        if let Some(mut next_node) = new_head {
            head_node.next = next_node.next.take();
            next_node.next = Some(head_node);
            Some(next_node)
        } else {
            Some(head_node)
        }
    } else {
        None
    }
}

// 反转链表的一部分 [left, right]
fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    if left == right {
        return head;
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev = &mut dummy;
    
    // 移动到需要反转的前一个位置
    for _ in 0..left - 1 {
        prev = prev.next.as_mut().unwrap();
    }
    
    // 反转 right - left + 1 个节点
    let mut current = prev.next.take();
    for _ in 0..right - left + 1 {
        if let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev.next.take();
            prev.next = Some(node);
            current = next;
        }
    }
    
    // 连接剩余部分
    let mut tail = &mut prev.next;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = current;
    
    dummy.next
}
```

### 4. 链表合并
```rust
// 合并两个有序链表
fn merge_two_lists(
    list1: Option<Box<ListNode>>, 
    list2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    let mut l1 = list1;
    let mut l2 = list2;
    
    while l1.is_some() && l2.is_some() {
        if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
            let next = l1.as_mut().unwrap().next.take();
            current.next = l1;
            l1 = next;
        } else {
            let next = l2.as_mut().unwrap().next.take();
            current.next = l2;
            l2 = next;
        }
        current = current.next.as_mut().unwrap();
    }
    
    // 连接剩余节点
    current.next = l1.or(l2);
    
    dummy.next
}
```

---

## 经典题目实现

### 1. LeetCode 21: 合并两个有序链表
```rust
// 已在上面实现
```

### 2. LeetCode 206: 反转链表
```rust
// 已在上面实现
```

### 3. LeetCode 141: 环形链表
```rust
// 已在上面实现
```

### 4. LeetCode 2: 两数相加
```rust
fn add_two_numbers(
    l1: Option<Box<ListNode>>, 
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    let mut carry = 0;
    let mut p1 = &l1;
    let mut p2 = &l2;
    
    while p1.is_some() || p2.is_some() || carry > 0 {
        let val1 = p1.as_ref().map_or(0, |node| node.val);
        let val2 = p2.as_ref().map_or(0, |node| node.val);
        
        let sum = val1 + val2 + carry;
        carry = sum / 10;
        
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        
        if let Some(node) = p1 {
            p1 = &node.next;
        }
        if let Some(node) = p2 {
            p2 = &node.next;
        }
    }
    
    dummy.next
}
```

---

## 调试技巧

### 1. 打印链表
```rust
fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;
    let mut result = Vec::new();
    
    while let Some(node) = current {
        result.push(node.val.to_string());
        current = &node.next;
    }
    
    println!("[{}]", result.join(" -> "));
}
```

### 2. 比较两个链表
```rust
fn lists_equal(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> bool {
    let mut p1 = l1;
    let mut p2 = l2;
    
    while p1.is_some() && p2.is_some() {
        if p1.as_ref().unwrap().val != p2.as_ref().unwrap().val {
            return false;
        }
        p1 = &p1.as_ref().unwrap().next;
        p2 = &p2.as_ref().unwrap().next;
    }
    
    p1.is_none() && p2.is_none()
}
```

### 3. 常用测试用例
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list() {
        let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
        print_list(&list);
    }

    #[test]
    fn test_reverse_list() {
        let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
        let reversed = reverse_list(list);
        print_list(&reversed); // 应该输出 [5 -> 4 -> 3 -> 2 -> 1]
    }

    #[test]
    fn test_find_middle() {
        let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
        let middle = find_middle(list);
        assert_eq!(middle.unwrap().val, 3);
    }
}
```

---

## 💡 关键要点

### 1. 所有权管理
- 使用 `Option<Box<ListNode>>` 表示可能为空的节点
- 使用 `take()` 方法获取所有权
- 注意避免所有权冲突

### 2. 常用模式
```rust
// 安全获取下一个节点
if let Some(node) = current {
    current = &node.next;
}

// 修改链表时的所有权转移
let next = current.as_mut().unwrap().next.take();
```

### 3. 虚拟头节点的优势
- 简化边界情况处理
- 统一插入/删除操作
- 避免特殊处理头节点

### 4. 递归 vs 迭代
- 递归：代码简洁，但可能栈溢出
- 迭代：更安全，性能更好，推荐使用

---

## 🔍 实战建议

1. **先画图**：复杂的链表操作前先画出链表结构
2. **虚拟头节点**：几乎所有需要修改链表的题目都建议使用
3. **双指针**：熟练掌握快慢指针和间距指针技巧
4. **边界条件**：空链表、单节点、两节点等情况
5. **所有权检查**：确保每个节点的所有权转移正确 