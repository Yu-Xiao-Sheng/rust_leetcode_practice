/**
 * 69. x 的平方根 
简单
相关标签
premium lock icon
相关企业
提示
给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。

 

示例 1：

输入：x = 4
输出：2
示例 2：

输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
 

提示：

0 <= x <= 231 - 1
 */
struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // 处理边界情况
        if x <= 1 {
            return x;
        }
        
        let mut left = 1;
        let mut right = x;
        
        while left < right {
            let mid = left + (right - left) / 2;
            
            // 避免整数溢出：使用除法而不是乘法
            if mid <= x / mid {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        // left 指向第一个 mid > x/mid 的位置，所以答案是 left - 1
        left - 1
    }
}

fn main() {
    println!("=== LeetCode 69. x 的平方根 ===\n");
    
    // 题目示例测试
    println!("--- 题目示例测试 ---");
    
    let x = 4;
    let result = Solution::my_sqrt(x);
    println!("示例1: x = 4");
    println!("期望输出: 2, 实际输出: {}", result);
    println!("验证: {} * {} = {}", result, result, result * result);
    println!();
    
    let x = 8;
    let result = Solution::my_sqrt(x);
    println!("示例2: x = 8");
    println!("期望输出: 2, 实际输出: {}", result);
    println!("验证: {} * {} = {} <= 8", result, result, result * result);
    println!("下一个数: {} * {} = {} > 8", result + 1, result + 1, (result + 1) * (result + 1));
    println!();
    
    // 边界情况测试
    println!("--- 边界情况测试 ---");
    
    // 1. 最小值和0
    let x = 0;
    let result = Solution::my_sqrt(x);
    println!("最小值测试: x = 0");
    println!("期望输出: 0, 实际输出: {}", result);
    println!("验证: {} * {} = {}", result, result, result * result);
    println!();
    
    let x = 1;
    let result = Solution::my_sqrt(x);
    println!("单个值测试: x = 1");
    println!("期望输出: 1, 实际输出: {}", result);
    println!("验证: {} * {} = {}", result, result, result * result);
    println!();
    
    // 2. 完全平方数
    let test_cases = vec![4, 9, 16, 25, 36, 49, 64, 81, 100];
    for &x in &test_cases {
        let result = Solution::my_sqrt(x);
        println!("完全平方数: x = {}", x);
        println!("期望输出: {}, 实际输出: {}", (x as f64).sqrt() as i32, result);
        println!("验证: {} * {} = {}", result, result, result * result);
        println!();
    }
    
    // 3. 非完全平方数
    let test_cases = vec![2, 3, 5, 6, 7, 10, 11, 12, 13, 14, 15];
    for &x in &test_cases {
        let result = Solution::my_sqrt(x);
        let expected = (x as f64).sqrt() as i32;
        println!("非完全平方数: x = {}", x);
        println!("期望输出: {}, 实际输出: {}", expected, result);
        println!("验证: {} * {} = {} <= {}", result, result, result * result, x);
        println!("下一个数: {} * {} = {} > {}", result + 1, result + 1, (result + 1) * (result + 1), x);
        println!();
    }
    
    // 4. 大数测试
    println!("--- 大数测试 ---");
    
    let x = 10000;
    let result = Solution::my_sqrt(x);
    println!("大数测试1: x = 10000");
    println!("期望输出: 100, 实际输出: {}", result);
    println!("验证: {} * {} = {}", result, result, result * result);
    println!();
    
    let x = 99999;
    let result = Solution::my_sqrt(x);
    println!("大数测试2: x = 99999");
    println!("期望输出: 316, 实际输出: {}", result);
    println!("验证: {} * {} = {} <= {}", result, result, result * result, x);
    println!("下一个数: {} * {} = {} > {}", result + 1, result + 1, (result + 1) * (result + 1), x);
    println!();
    
    // 5. 边界值测试
    println!("--- 边界值测试 ---");
    
    let x = i32::MAX;
    let result = Solution::my_sqrt(x);
    println!("最大整数测试: x = {}", x);
    println!("期望输出: 46340, 实际输出: {}", result);
    println!("验证: {} * {} = {} <= {}", result, result, result * result, x);
    println!("下一个数: {} * {} = {} > {}", result + 1, result + 1, (result + 1) * (result + 1), x);
    println!();
    
    // 6. 特殊值测试
    println!("--- 特殊值测试 ---");
    
    let test_cases = vec![
        (2, 1),      // √2 ≈ 1.414
        (3, 1),      // √3 ≈ 1.732
        (5, 2),      // √5 ≈ 2.236
        (6, 2),      // √6 ≈ 2.449
        (7, 2),      // √7 ≈ 2.645
        (10, 3),     // √10 ≈ 3.162
        (15, 3),     // √15 ≈ 3.872
        (20, 4),     // √20 ≈ 4.472
        (30, 5),     // √30 ≈ 5.477
        (40, 6),     // √40 ≈ 6.324
        (50, 7),     // √50 ≈ 7.071
    ];
    
    for &(x, expected) in &test_cases {
        let result = Solution::my_sqrt(x);
        println!("特殊值: x = {}, 期望: {}, 实际: {}", x, expected, result);
        println!("验证: {} * {} = {} <= {}", result, result, result * result, x);
        if result != expected {
            println!("❌ 结果不匹配！");
        } else {
            println!("✅ 结果正确！");
        }
        println!();
    }
    
    // 7. 性能测试 - 大范围测试
    println!("--- 性能测试 ---");
    
    let test_values = vec![
        100, 1000, 10000, 100000, 1000000, 10000000
    ];
    
    for &x in &test_values {
        let result = Solution::my_sqrt(x);
        let expected = (x as f64).sqrt() as i32;
        println!("性能测试: x = {}, 期望: {}, 实际: {}", x, expected, result);
        if result == expected {
            println!("✅ 正确");
        } else {
            println!("❌ 错误");
        }
    }
    println!();
    
    // 8. 边界情况：接近完全平方数
    println!("--- 接近完全平方数测试 ---");
    
    let test_cases = vec![
        (3, 1),      // 接近 2² = 4
        (8, 2),      // 接近 3² = 9
        (15, 3),     // 接近 4² = 16
        (24, 4),     // 接近 5² = 25
        (35, 5),     // 接近 6² = 36
        (48, 6),     // 接近 7² = 49
        (63, 7),     // 接近 8² = 64
        (80, 8),     // 接近 9² = 81
        (99, 9),     // 接近 10² = 100
    ];
    
    for &(x, expected) in &test_cases {
        let result = Solution::my_sqrt(x);
        println!("接近完全平方数: x = {}, 期望: {}, 实际: {}", x, expected, result);
        println!("验证: {} * {} = {} <= {} < {} * {} = {}", 
                result, result, result * result, x, result + 1, result + 1, (result + 1) * (result + 1));
        if result == expected {
            println!("✅ 正确");
        } else {
            println!("❌ 错误");
        }
        println!();
    }
    
    // 9. 算法验证测试
    println!("--- 算法验证测试 ---");
    
    for x in 0..=20 {
        let result = Solution::my_sqrt(x);
        let expected = (x as f64).sqrt() as i32;
        println!("x = {:2}, 期望: {:2}, 实际: {:2}, 验证: {}² = {} <= {} < {}² = {}", 
                x, expected, result, 
                result, result * result, x, 
                result + 1, (result + 1) * (result + 1));
    }
}