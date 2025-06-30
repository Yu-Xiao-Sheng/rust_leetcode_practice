/**
 * 34. 在排序数组中查找元素的第一个和最后一个位置
中等
相关标签
premium lock icon
相关企业
给你一个按照非递减顺序排列的整数数组 nums，和一个目标值 target。请你找出给定目标值在数组中的开始位置和结束位置。

如果数组中不存在目标值 target，返回 [-1, -1]。

你必须设计并实现时间复杂度为 O(log n) 的算法解决此问题。

 

示例 1：

输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]
示例 2：

输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]
示例 3：

输入：nums = [], target = 0
输出：[-1,-1]
 

提示：

0 <= nums.length <= 105
-109 <= nums[i] <= 109
nums 是一个非递减数组
-109 <= target <= 109
 */

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            }else{
                right = mid;
            }
        }

        if left < nums.len() && nums[left] == target {
            result.push(left as i32);
        }else{
            result.push(-1);
        }

        left = 0;
        right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            }else {
                right = mid;
            }
        }
        if left > 0 && nums[left-1] == target {
            result.push((left-1) as i32);
        }else {
            result.push(-1);
        }
        result
    }
}

fn main() {
    println!("=== LeetCode 34. 在排序数组中查找元素的第一个和最后一个位置 ===\n");
    
    // 题目示例测试
    println!("--- 题目示例测试 ---");
    
    let nums = vec![5,7,7,8,8,10];
    let target = 8;
    let result = Solution::search_range(nums, target);
    println!("示例1: nums = [5,7,7,8,8,10], target = 8");
    println!("期望输出: [3,4], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![5,7,7,8,8,10];
    let target = 6;
    let result = Solution::search_range(nums, target);
    println!("示例2: nums = [5,7,7,8,8,10], target = 6");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![];
    let target = 0;
    let result = Solution::search_range(nums, target);
    println!("示例3: nums = [], target = 0");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    // 边界情况测试
    println!("--- 边界情况测试 ---");
    
    // 1. 单个元素数组
    let nums = vec![5];
    let target = 5;
    let result = Solution::search_range(nums, target);
    println!("单个元素且等于目标值: nums = [5], target = 5");
    println!("期望输出: [0,0], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![5];
    let target = 3;
    let result = Solution::search_range(nums, target);
    println!("单个元素且小于目标值: nums = [5], target = 3");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![5];
    let target = 7;
    let result = Solution::search_range(nums, target);
    println!("单个元素且大于目标值: nums = [5], target = 7");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    // 2. 两个元素数组
    let nums = vec![5, 5];
    let target = 5;
    let result = Solution::search_range(nums, target);
    println!("两个相同元素: nums = [5,5], target = 5");
    println!("期望输出: [0,1], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![5, 7];
    let target = 5;
    let result = Solution::search_range(nums, target);
    println!("两个不同元素，目标值在开头: nums = [5,7], target = 5");
    println!("期望输出: [0,0], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![5, 7];
    let target = 7;
    let result = Solution::search_range(nums, target);
    println!("两个不同元素，目标值在末尾: nums = [5,7], target = 7");
    println!("期望输出: [1,1], 实际输出: {:?}", result);
    println!();
    
    // 3. 目标值在数组边界
    let nums = vec![1, 2, 2, 2, 3, 4, 5];
    let target = 1;
    let result = Solution::search_range(nums, target);
    println!("目标值在数组开头: nums = [1,2,2,2,3,4,5], target = 1");
    println!("期望输出: [0,0], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![1, 2, 2, 2, 3, 4, 5];
    let target = 5;
    let result = Solution::search_range(nums, target);
    println!("目标值在数组末尾: nums = [1,2,2,2,3,4,5], target = 5");
    println!("期望输出: [6,6], 实际输出: {:?}", result);
    println!();
    
    // 4. 多个相同元素
    let nums = vec![2, 2, 2, 2, 2];
    let target = 2;
    let result = Solution::search_range(nums, target);
    println!("所有元素都相同: nums = [2,2,2,2,2], target = 2");
    println!("期望输出: [0,4], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![1, 2, 2, 2, 3];
    let target = 2;
    let result = Solution::search_range(nums, target);
    println!("中间有多个相同元素: nums = [1,2,2,2,3], target = 2");
    println!("期望输出: [1,3], 实际输出: {:?}", result);
    println!();
    
    // 5. 目标值不存在，需要插入到不同位置
    let nums = vec![1, 3, 5, 7, 9];
    let target = 0;  // 插入到开头
    let result = Solution::search_range(nums, target);
    println!("目标值小于所有元素: nums = [1,3,5,7,9], target = 0");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![1, 3, 5, 7, 9];
    let target = 4;  // 插入到中间
    let result = Solution::search_range(nums, target);
    println!("目标值在中间但不存在: nums = [1,3,5,7,9], target = 4");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![1, 3, 5, 7, 9];
    let target = 10;  // 插入到末尾
    let result = Solution::search_range(nums, target);
    println!("目标值大于所有元素: nums = [1,3,5,7,9], target = 10");
    println!("期望输出: [-1,-1], 实际输出: {:?}", result);
    println!();
    
    // 6. 负数测试
    let nums = vec![-10, -5, -5, -5, 0, 5, 10];
    let target = -5;
    let result = Solution::search_range(nums, target);
    println!("负数测试: nums = [-10,-5,-5,-5,0,5,10], target = -5");
    println!("期望输出: [1,3], 实际输出: {:?}", result);
    println!();
    
    // 7. 大数组测试
    let nums: Vec<i32> = (0..100).map(|x| x / 10).collect();  // [0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,2,2,...]
    let target = 5;
    let result = Solution::search_range(nums, target);
    println!("大数组测试: nums = [0,0,...,1,1,...,2,2,...], target = 5");
    println!("期望输出: [50,59], 实际输出: {:?}", result);
    println!();
    
    // 8. 边界值测试
    let nums = vec![i32::MIN, -1000, 0, 1000, i32::MAX];
    let target = i32::MIN;
    let result = Solution::search_range(nums, target);
    println!("最小整数测试: nums = [MIN,-1000,0,1000,MAX], target = MIN");
    println!("期望输出: [0,0], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![i32::MIN, -1000, 0, 1000, i32::MAX];
    let target = i32::MAX;
    let result = Solution::search_range(nums, target);
    println!("最大整数测试: nums = [MIN,-1000,0,1000,MAX], target = MAX");
    println!("期望输出: [4,4], 实际输出: {:?}", result);
    println!();
    
    // 9. 特殊情况：只有一个目标值
    let nums = vec![1, 2, 3, 4, 5];
    let target = 3;
    let result = Solution::search_range(nums, target);
    println!("只有一个目标值: nums = [1,2,3,4,5], target = 3");
    println!("期望输出: [2,2], 实际输出: {:?}", result);
    println!();
    
    // 10. 目标值在数组开头和末尾
    let nums = vec![8, 8, 8, 8, 8];
    let target = 8;
    let result = Solution::search_range(nums, target);
    println!("目标值占据整个数组: nums = [8,8,8,8,8], target = 8");
    println!("期望输出: [0,4], 实际输出: {:?}", result);
    println!();
    
    // 11. 复杂情况：多个不同目标值
    let nums = vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4];
    let target = 2;
    let result = Solution::search_range(nums, target);
    println!("复杂情况: nums = [1,1,2,2,2,3,3,4,4,4,4], target = 2");
    println!("期望输出: [2,4], 实际输出: {:?}", result);
    println!();
    
    let nums = vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4];
    let target = 4;
    let result = Solution::search_range(nums, target);
    println!("复杂情况: nums = [1,1,2,2,2,3,3,4,4,4,4], target = 4");
    println!("期望输出: [7,10], 实际输出: {:?}", result);
    println!();
}

