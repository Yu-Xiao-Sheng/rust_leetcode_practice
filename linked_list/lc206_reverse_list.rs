/**
 * LeetCode 206: 反转链表
 * 
 * 给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
 * 
 * 示例 1:
 * 输入：head = [1,2,3,4,5]
 * 输出：[5,4,3,2,1]
 * 
 * 示例 2:
 * 输入：head = [1,2]
 * 输出：[2,1]
 * 
 * 示例 3:
 * 输入：head = []
 * 输出：[]
 */

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

pub struct Solution;

impl Solution {
    /// 方法1：迭代法（推荐）
    /// 时间复杂度：O(n)
    /// 空间复杂度：O(1)
    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;           // 前一个节点
        let mut current = head;        // 当前节点
        
        while let Some(mut node) = current {
            let next = node.next.take();  // 保存下一个节点
            node.next = prev;              // 反转指针
            prev = Some(node);             // 前一个节点向前移动
            current = next;                // 当前节点向前移动
        }
        
        prev  // 新的头节点
    }
    
    /// 方法2：递归法
    /// 时间复杂度：O(n)
    /// 空间复杂度：O(n) - 递归栈深度
    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_helper(head)
    }
    
    fn reverse_helper(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 基础情况：空节点或只有一个节点
        match head {
            None => None,
            Some(mut node) => {
                match node.next.take() {
                    None => Some(node),  // 只有一个节点，直接返回
                    Some(next) => {
                        // 递归反转剩余部分
                        let new_head = Self::reverse_helper(Some(next));
                        
                        // 反转当前连接
                        // 注意：这里需要找到新链表的尾部来连接当前节点
                        // 但这种方法效率不高，更好的递归实现如下：
                        Self::connect_reversed(new_head, Some(node))
                    }
                }
            }
        }
    }
    
    /// 辅助函数：连接已反转的链表和当前节点
    fn connect_reversed(
        reversed: Option<Box<ListNode>>, 
        current: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        if let Some(mut curr) = current {
            curr.next = reversed;
            Some(curr)
        } else {
            reversed
        }
    }
    
    /// 方法3：更优雅的递归实现
    /// 使用尾递归的思想
    pub fn reverse_list_tail_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_with_prev(head, None)
    }
    
    fn reverse_with_prev(
        current: Option<Box<ListNode>>, 
        prev: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        match current {
            None => prev,
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                Self::reverse_with_prev(next, Some(node))
            }
        }
    }
}

// 辅助函数：从数组创建链表
pub fn create_list_from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    
    for val in vals {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    dummy.next
}

// 辅助函数：将链表转换为数组（用于测试）
pub fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    
    result
}

// 辅助函数：打印链表
pub fn print_list(head: &Option<Box<ListNode>>) {
    let values = list_to_vec(head);
    println!("{:?}", values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list_iterative() {
        // 测试用例1: [1,2,3,4,5] -> [5,4,3,2,1]
        let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
        let reversed = Solution::reverse_list_iterative(list);
        assert_eq!(list_to_vec(&reversed), vec![5, 4, 3, 2, 1]);
        
        // 测试用例2: [1,2] -> [2,1]
        let list = create_list_from_vec(vec![1, 2]);
        let reversed = Solution::reverse_list_iterative(list);
        assert_eq!(list_to_vec(&reversed), vec![2, 1]);
        
        // 测试用例3: [] -> []
        let list = create_list_from_vec(vec![]);
        let reversed = Solution::reverse_list_iterative(list);
        assert_eq!(list_to_vec(&reversed), vec![]);
        
        // 测试用例4: [1] -> [1]
        let list = create_list_from_vec(vec![1]);
        let reversed = Solution::reverse_list_iterative(list);
        assert_eq!(list_to_vec(&reversed), vec![1]);
    }

    #[test]
    fn test_reverse_list_tail_recursive() {
        // 测试用例1: [1,2,3,4,5] -> [5,4,3,2,1]
        let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
        let reversed = Solution::reverse_list_tail_recursive(list);
        assert_eq!(list_to_vec(&reversed), vec![5, 4, 3, 2, 1]);
        
        // 测试用例2: [1,2] -> [2,1]
        let list = create_list_from_vec(vec![1, 2]);
        let reversed = Solution::reverse_list_tail_recursive(list);
        assert_eq!(list_to_vec(&reversed), vec![2, 1]);
        
        // 测试用例3: [] -> []
        let list = create_list_from_vec(vec![]);
        let reversed = Solution::reverse_list_tail_recursive(list);
        assert_eq!(list_to_vec(&reversed), vec![]);
    }

    #[test]
    fn test_performance_comparison() {
        // 创建较大的链表进行性能测试
        let large_list: Vec<i32> = (1..=1000).collect();
        
        let list1 = create_list_from_vec(large_list.clone());
        let list2 = create_list_from_vec(large_list.clone());
        
        let start = std::time::Instant::now();
        let _reversed1 = Solution::reverse_list_iterative(list1);
        let iter_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let _reversed2 = Solution::reverse_list_tail_recursive(list2);
        let rec_time = start.elapsed();
        
        println!("迭代法耗时: {:?}", iter_time);
        println!("递归法耗时: {:?}", rec_time);
    }
}

/// 主函数示例
fn main() {
    println!("=== LeetCode 206: 反转链表 ===");
    
    // 示例1
    let list1 = create_list_from_vec(vec![1, 2, 3, 4, 5]);
    println!("原链表: ");
    print_list(&list1);
    
    let reversed1 = Solution::reverse_list_iterative(list1);
    println!("反转后(迭代法): ");
    print_list(&reversed1);
    
    // 示例2
    let list2 = create_list_from_vec(vec![1, 2]);
    println!("\n原链表: ");
    print_list(&list2);
    
    let reversed2 = Solution::reverse_list_tail_recursive(list2);
    println!("反转后(递归法): ");
    print_list(&reversed2);
} 