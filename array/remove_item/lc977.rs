#!/usr/bin/env rust-script

/*
977. 有序数组的平方
简单
相关标签
premium lock icon
相关企业
给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。

 

示例 1：

输入：nums = [-4,-1,0,3,10]
输出：[0,1,9,16,100]
解释：平方后，数组变为 [16,1,0,9,100]
排序后，数组变为 [0,1,9,16,100]
示例 2：

输入：nums = [-7,-3,2,3,11]
输出：[4,9,9,49,121]
 

提示：

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums 已按 非递减顺序 排序
 

进阶：

请你设计时间复杂度为 O(n) 的算法解决本问题
*/

struct Solution;

impl Solution {
    // 解法一：暴力解法 - 平方后排序
    // 时间复杂度：O(n log n)，空间复杂度：O(1)
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = nums.iter().map(|&x| x * x).collect();
        result.sort();
        result
    }
    
    // 解法二：双指针法 - O(n) 时间复杂度
    // 时间复杂度：O(n)，空间复杂度：O(n)
    pub fn sorted_squares_two_pointers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];  // 创建结果数组
        
        let mut left = 0;           // 左指针，指向数组开头
        let mut right = n - 1;      // 右指针，指向数组末尾
        let mut pos = n - 1;        // 结果数组的填充位置（从后往前填）
        
        // 关键思路：最大的平方值一定在数组的两端！
        // 因为负数的平方可能很大，正数的平方也可能很大
        while left <= right {
            let left_square = nums[left] * nums[left];   // 左端平方值
            let right_square = nums[right] * nums[right]; // 右端平方值
            
            if left_square > right_square {
                // 左端的平方更大，放入结果数组
                result[pos] = left_square;
                left += 1;  // 左指针右移
            } else {
                // 右端的平方更大（或相等），放入结果数组
                result[pos] = right_square;
                right -= 1; // 右指针左移
            }
            
            if pos == 0 { break; }  // 防止溢出
            pos -= 1;  // 结果数组位置前移
        }
        
        result
    }
    
    // 解法三：你的思路2的优化版本 - 找最小值然后向两边扩展
    // 时间复杂度：O(n)，空间复杂度：O(n)
    pub fn sorted_squares_expand_from_center(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = Vec::new();
        
        // 第一步：找到绝对值最小的元素位置
        let mut min_abs_idx = 0;
        let mut min_abs_val = nums[0].abs();
        
        for i in 1..n {
            if nums[i].abs() < min_abs_val {
                min_abs_val = nums[i].abs();
                min_abs_idx = i;
            }
        }
        
        // 第二步：从最小绝对值位置向两边扩展
        let mut left = min_abs_idx as i32;
        let mut right = min_abs_idx as i32;
        
        // 先添加中心元素
        result.push(nums[min_abs_idx] * nums[min_abs_idx]);
        left -= 1;
        right += 1;
        
        // 向两边扩展，每次选择绝对值较小的
        while left >= 0 || right < n as i32 {
            let left_val = if left >= 0 { Some(nums[left as usize].abs()) } else { None };
            let right_val = if right < n as i32 { Some(nums[right as usize].abs()) } else { None };
            
            match (left_val, right_val) {
                (Some(l), Some(r)) => {
                    if l <= r {
                        result.push(nums[left as usize] * nums[left as usize]);
                        left -= 1;
                    } else {
                        result.push(nums[right as usize] * nums[right as usize]);
                        right += 1;
                    }
                }
                (Some(_), None) => {
                    result.push(nums[left as usize] * nums[left as usize]);
                    left -= 1;
                }
                (None, Some(_)) => {
                    result.push(nums[right as usize] * nums[right as usize]);
                    right += 1;
                }
                (None, None) => break,
            }
        }
        
        result
    }

    pub fn sorted_squares2(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = vec![0; nums.len()];
        let mut pos = nums.len() - 1;

        while left <= right {
            let left_result = nums[left] * nums[left];
            let right_result = nums[right] * nums[right];
            if left_result > right_result {
                result[pos] = left_result;
                left+=1;
            } else {
                result[pos] = right_result;
                right-=1;
            }
            if pos <= 0 {
                break;
            }
            pos -= 1;

        }

        result
    }

}

fn main() {
    println!("=== LeetCode 977: 有序数组的平方 ===\n");
    
    // 测试用例1
    let nums1 = vec![-4, -1, 0, 3, 10];
    println!("测试1: {:?}", nums1);
    println!("期望输出: [0, 1, 9, 16, 100]");
    
    let result1_1 = Solution::sorted_squares(nums1.clone());
    let result1_2 = Solution::sorted_squares2(nums1.clone());
    let result1_3 = Solution::sorted_squares_expand_from_center(nums1.clone());
    
    println!("暴力解法:   {:?}", result1_1);
    println!("双指针法:   {:?}", result1_2);
    println!("中心扩展法: {:?}", result1_3);
    println!();
    
    // 测试用例2
    let nums2 = vec![-7, -3, 2, 3, 11];
    println!("测试2: {:?}", nums2);
    println!("期望输出: [4, 9, 9, 49, 121]");
    
    let result2_1 = Solution::sorted_squares(nums2.clone());
    let result2_2 = Solution::sorted_squares_two_pointers(nums2.clone());
    let result2_3 = Solution::sorted_squares_expand_from_center(nums2.clone());
    
    println!("暴力解法:   {:?}", result2_1);
    println!("双指针法:   {:?}", result2_2);
    println!("中心扩展法: {:?}", result2_3);
    println!();
    
    // 测试用例3：全负数
    let nums3 = vec![-5, -3, -2, -1];
    println!("测试3: {:?}", nums3);
    println!("期望输出: [1, 4, 9, 25]");
    
    let result3_1 = Solution::sorted_squares(nums3.clone());
    let result3_2 = Solution::sorted_squares_two_pointers(nums3.clone());
    let result3_3 = Solution::sorted_squares_expand_from_center(nums3.clone());
    
    println!("暴力解法:   {:?}", result3_1);
    println!("双指针法:   {:?}", result3_2);
    println!("中心扩展法: {:?}", result3_3);
    println!();
    
    // 测试用例4：全正数
    let nums4 = vec![1, 2, 3, 4, 5];
    println!("测试4: {:?}", nums4);
    println!("期望输出: [1, 4, 9, 16, 25]");
    
    let result4_1 = Solution::sorted_squares(nums4.clone());
    let result4_2 = Solution::sorted_squares_two_pointers(nums4.clone());
    let result4_3 = Solution::sorted_squares_expand_from_center(nums4.clone());
    
    println!("暴力解法:   {:?}", result4_1);
    println!("双指针法:   {:?}", result4_2);
    println!("中心扩展法: {:?}", result4_3);
}

/*
算法详解：

=== 核心洞察 ===
1. 原数组已经有序，但平方后可能无序（因为负数）
2. 平方后，最小值在中间某处，最大值一定在两端
3. 关键是如何高效地构造有序的结果

=== 解法一：暴力解法 ===
- 直接平方所有元素，然后排序
- 简单直接，但时间复杂度 O(n log n)

=== 解法二：双指针法（推荐） ===
思路：
1. 最大的平方值一定在数组两端（最左或最右）
2. 用双指针分别指向头尾
3. 比较两端平方值，将较大者放入结果数组末尾
4. 移动对应指针，继续比较

步骤演示（以 [-4,-1,0,3,10] 为例）：
初始: left=0(-4), right=4(10), pos=4
比较: (-4)²=16 vs (10)²=100 → 100更大 → result[4]=100, right--
比较: (-4)²=16 vs (3)²=9   → 16更大  → result[3]=16,  left++
比较: (-1)²=1  vs (3)²=9   → 9更大   → result[2]=9,   right--
比较: (-1)²=1  vs (0)²=0   → 1更大   → result[1]=1,   left++
最后: (0)²=0               → 0       → result[0]=0

最终结果: [0,1,9,16,100]

=== 解法三：中心扩展法 ===
思路：
1. 找到绝对值最小的元素（平方后最小）
2. 从该位置向两边扩展
3. 每次选择绝对值较小的元素

这个方法实现了你的想法2，但代码稍微复杂一些。

=== 复杂度对比 ===
| 解法     | 时间复杂度 | 空间复杂度 | 优缺点 |
|----------|------------|------------|---------|
| 暴力解法 | O(n log n) | O(1)       | 简单但不是最优 |
| 双指针法 | O(n)       | O(n)       | 最优解，推荐 |
| 中心扩展 | O(n)       | O(n)       | 符合直觉但实现复杂 |

=== 为什么双指针法是最优的？ ===
1. 利用了"最大值在两端"的性质
2. 一次遍历即可完成
3. 代码简洁易懂
4. 时间复杂度 O(n)，满足进阶要求
*/