// 链表模块入口文件

pub mod lc206_reverse_list;
pub mod lc21_merge_lists;

// 重新导出常用的结构和函数
pub use lc206_reverse_list::{ListNode, create_list_from_vec, list_to_vec, print_list};

#[cfg(test)]
mod tests {
    use super::*;
    use lc206_reverse_list::Solution as ReverseSolution;
    use lc21_merge_lists::Solution as MergeSolution;

    #[test]
    fn test_linked_list_basic_operations() {
        // 测试链表创建
        let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
        println!("创建的链表:");
        print_list(&list);
        assert_eq!(list_to_vec(&list), vec![1, 2, 3, 4, 5]);

        // 测试反转链表
        let reversed = ReverseSolution::reverse_list_iterative(list);
        println!("反转后的链表:");
        print_list(&reversed);
        assert_eq!(list_to_vec(&reversed), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_merge_sorted_lists() {
        let l1 = create_list_from_vec(vec![1, 2, 4]);
        let l2 = create_list_from_vec(vec![1, 3, 4]);
        
        println!("合并前:");
        print!("链表1: ");
        print_list(&l1);
        print!("链表2: ");
        print_list(&l2);
        
        let merged = MergeSolution::merge_two_lists(l1, l2);
        println!("合并后:");
        print_list(&merged);
        
        assert_eq!(list_to_vec(&merged), vec![1, 1, 2, 3, 4, 4]);
    }
}

/// 展示链表基本用法的示例函数
pub fn demo_linked_list_usage() {
    println!("=== Rust 链表使用示例 ===\n");
    
    // 1. 创建链表
    println!("1. 创建链表 [1, 2, 3, 4, 5]:");
    let list = create_list_from_vec(vec![1, 2, 3, 4, 5]);
    print_list(&list);
    
    // 2. 反转链表
    println!("\n2. 反转链表:");
    let reversed = lc206_reverse_list::Solution::reverse_list_iterative(list);
    print_list(&reversed);
    
    // 3. 合并两个有序链表
    println!("\n3. 合并两个有序链表:");
    let l1 = create_list_from_vec(vec![1, 3, 5]);
    let l2 = create_list_from_vec(vec![2, 4, 6]);
    
    print!("链表1: ");
    print_list(&l1);
    print!("链表2: ");
    print_list(&l2);
    
    let merged = lc21_merge_lists::Solution::merge_two_lists(l1, l2);
    print!("合并结果: ");
    print_list(&merged);
    
    println!("\n=== 示例完成 ===");
} 