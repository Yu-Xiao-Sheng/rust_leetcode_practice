/**
 * 367. 有效的完全平方数
简单
相关标签
premium lock icon
相关企业
给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。

完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。

不能使用任何内置的库函数，如  sqrt 。



示例 1：

输入：num = 16
输出：true
解释：返回 true ，因为 4 * 4 = 16 且 4 是一个整数。
示例 2：

输入：num = 14
输出：false
解释：返回 false ，因为 3.742 * 3.742 = 14 但 3.742 不是一个整数。


提示：

1 <= num <= 231 - 1
 */

// 引入测试验证器模块（从上层目录）
mod test_validator {
    include!("../test_validator.rs");
}
use test_validator::{TestGroup, TestSuite};

struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num <= 1 {
            return true;
        }

        let mut left = 1;
        let mut right = num;
        while left < right {
            let mid = left + (right - left) / 2;
            if mid <= num / mid {
                left = mid + 1; // 小于继续往右，最终left-1就是结果附近
            } else {
                right = mid;
            }
        }
        if (left - 1) * (left - 1) == num {
            return true;
        }
        false
    }
}

// 辅助函数：判断是否为完全平方数
fn is_perfect_square_naive(num: i32) -> bool {
    if num <= 1 {
        return true;
    }
    let sqrt = (num as f64).sqrt() as i32;
    sqrt * sqrt == num
}

fn main() {
    let mut test_suite = TestSuite::new("LeetCode 367. 有效的完全平方数");
    
    // 题目示例测试
    let mut group = TestGroup::new("题目示例测试");
    test_case_bool!(group.validator(), "示例1: num = 16", Solution::is_perfect_square(16), true);
    test_case_bool!(group.validator(), "示例2: num = 14", Solution::is_perfect_square(14), false);
    test_suite.add_group(group);
    
    // 边界情况测试
    let mut group = TestGroup::new("边界情况测试");
    test_case_bool!(group.validator(), "最小值: num = 1", Solution::is_perfect_square(1), true);
    test_case_bool!(group.validator(), "最小值: num = 0", Solution::is_perfect_square(0), true);
    test_case_bool!(group.validator(), "最小值: num = 2", Solution::is_perfect_square(2), false);
    test_case_bool!(group.validator(), "最小值: num = 3", Solution::is_perfect_square(3), false);
    test_suite.add_group(group);
    
    // 完全平方数测试
    let mut group = TestGroup::new("完全平方数测试");
    let perfect_squares = vec![4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225];
    for &num in &perfect_squares {
        test_case_bool!(
            group.validator(),
            &format!("完全平方数: num = {}", num),
            Solution::is_perfect_square(num),
            true
        );
    }
    test_suite.add_group(group);
    
    // 非完全平方数测试
    let mut group = TestGroup::new("非完全平方数测试");
    let non_perfect_squares = vec![2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20];
    for &num in &non_perfect_squares {
        test_case_bool!(
            group.validator(),
            &format!("非完全平方数: num = {}", num),
            Solution::is_perfect_square(num),
            false
        );
    }
    test_suite.add_group(group);
    
    // 大数测试
    let mut group = TestGroup::new("大数测试");
    let large_numbers = vec![
        (10000, true),   // 100²
        (10201, true),   // 101²
        (10404, true),   // 102²
        (10001, false),  // 不是完全平方数
        (10002, false),  // 不是完全平方数
        (10003, false),  // 不是完全平方数
    ];
    
    for &(num, expected) in &large_numbers {
        test_case_bool!(
            group.validator(),
            &format!("大数测试: num = {}", num),
            Solution::is_perfect_square(num),
            expected
        );
    }
    test_suite.add_group(group);
    
    // 边界值测试
    let mut group = TestGroup::new("边界值测试");
    let boundary_tests = vec![
        (i32::MAX, false),  // 最大整数不是完全平方数
        (46340 * 46340, true),  // 最大可能的完全平方数
        (46340 * 46340 + 1, false),  // 超过最大完全平方数
    ];
    
    for &(num, expected) in &boundary_tests {
        test_case_bool!(
            group.validator(),
            &format!("边界值: num = {}", num),
            Solution::is_perfect_square(num),
            expected
        );
    }
    test_suite.add_group(group);
    
    // 特殊值测试
    let mut group = TestGroup::new("特殊值测试");
    let special_tests = vec![
        (4, true),      // 2²
        (9, true),      // 3²
        (16, true),     // 4²
        (25, true),     // 5²
        (36, true),     // 6²
        (49, true),     // 7²
        (64, true),     // 8²
        (81, true),     // 9²
        (100, true),    // 10²
        (121, true),    // 11²
        (144, true),    // 12²
        (169, true),    // 13²
        (196, true),    // 14²
        (225, true),    // 15²
    ];
    
    for &(num, expected) in &special_tests {
        test_case_bool!(
            group.validator(),
            &format!("特殊值: num = {}", num),
            Solution::is_perfect_square(num),
            expected
        );
    }
    test_suite.add_group(group);
    
    // 接近完全平方数的测试
    let mut group = TestGroup::new("接近完全平方数测试");
    let near_perfect_squares = vec![
        (3, false),     // 接近 2² = 4
        (8, false),     // 接近 3² = 9
        (15, false),    // 接近 4² = 16
        (24, false),    // 接近 5² = 25
        (35, false),    // 接近 6² = 36
        (48, false),    // 接近 7² = 49
        (63, false),    // 接近 8² = 64
        (80, false),    // 接近 9² = 81
        (99, false),    // 接近 10² = 100
    ];
    
    for &(num, expected) in &near_perfect_squares {
        test_case_bool!(
            group.validator(),
            &format!("接近完全平方数: num = {}", num),
            Solution::is_perfect_square(num),
            expected
        );
    }
    test_suite.add_group(group);
    
    // 性能测试 - 大范围测试
    let mut group = TestGroup::new("性能测试");
    let performance_tests = vec![
        100, 1000, 10000, 100000, 1000000
    ];
    
    for &num in &performance_tests {
        let expected = is_perfect_square_naive(num);
        test_case_bool!(
            group.validator(),
            &format!("性能测试: num = {}", num),
            Solution::is_perfect_square(num),
            expected
        );
    }
    test_suite.add_group(group);
    
    // 算法验证测试 - 连续测试
    let mut group = TestGroup::new("算法验证测试");
    for num in 0..=50 {
        let expected = is_perfect_square_naive(num);
        let actual = Solution::is_perfect_square(num);
        if actual != expected {
            println!("❌ num = {}: 期望 {}, 实际 {}", num, expected, actual);
            group.validator().increment_failed();
        } else {
            group.validator().increment_passed();
        }
    }
    println!("连续测试 0-50 完成");
    test_suite.add_group(group);
    
    // 打印最终测试统计
    test_suite.print_final_summary();
}
