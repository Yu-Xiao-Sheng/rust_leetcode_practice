/**
 * 
代码
测试用例
测试结果
测试结果
35. 搜索插入位置
简单
相关标签
premium lock icon
相关企业
给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

请必须使用时间复杂度为 O(log n) 的算法。

 

示例 1:

输入: nums = [1,3,5,6], target = 5
输出: 2
示例 2:

输入: nums = [1,3,5,6], target = 2
输出: 1
示例 3:

输入: nums = [1,3,5,6], target = 7
输出: 4
 

提示:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums 为 无重复元素 的 升序 排列数组
-104 <= target <= 104
 */

// 添加 Solution 结构体定义
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right -left) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            }else{
                right = mid;
            }
        }

        if left > 0 && nums[left-1] == target{
            return (left-1) as i32;
        }
        left as i32
    }
}

fn main() {
    // 测试用例1
    let nums = vec![1,3,5,6];
    let target = 5;
    let result = Solution::search_insert(nums, target);
    println!("测试用例1: nums = [1,3,5,6], target = 5");
    println!("期望输出: 2, 实际输出: {}", result);
    println!();
    
    // 测试用例2
    let nums = vec![1,3,5,6];
    let target = 2;
    let result = Solution::search_insert(nums, target);
    println!("测试用例2: nums = [1,3,5,6], target = 2");
    println!("期望输出: 1, 实际输出: {}", result);
    println!();
    
    // 测试用例3
    let nums = vec![1,3,5,6];
    let target = 7;
    let result = Solution::search_insert(nums, target);
    println!("测试用例3: nums = [1,3,5,6], target = 7");
    println!("期望输出: 4, 实际输出: {}", result);
    println!();
    
    // 额外测试用例
    let nums = vec![1,3,5,6];
    let target = 0;
    let result = Solution::search_insert(nums, target);
    println!("额外测试: nums = [1,3,5,6], target = 0");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    // 刁钻测试用例
    println!("=== 刁钻测试用例 ===");
    
    // 1. 空数组
    let nums = vec![];
    let target = 5;
    let result = Solution::search_insert(nums, target);
    println!("空数组测试: nums = [], target = 5");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    // 2. 单个元素数组
    let nums = vec![5];
    let target = 5;
    let result = Solution::search_insert(nums, target);
    println!("单个元素且等于目标值: nums = [5], target = 5");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    let nums = vec![5];
    let target = 3;
    let result = Solution::search_insert(nums, target);
    println!("单个元素且小于目标值: nums = [5], target = 3");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    let nums = vec![5];
    let target = 7;
    let result = Solution::search_insert(nums, target);
    println!("单个元素且大于目标值: nums = [5], target = 7");
    println!("期望输出: 1, 实际输出: {}", result);
    println!();
    
    // 3. 目标值在数组边界
    let nums = vec![1, 3, 5, 7, 9];
    let target = 1;
    let result = Solution::search_insert(nums, target);
    println!("目标值在数组开头: nums = [1,3,5,7,9], target = 1");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    let nums = vec![1, 3, 5, 7, 9];
    let target = 9;
    let result = Solution::search_insert(nums, target);
    println!("目标值在数组末尾: nums = [1,3,5,7,9], target = 9");
    println!("期望输出: 4, 实际输出: {}", result);
    println!();
    
    // 4. 目标值在数组中间
    let nums = vec![1, 3, 5, 7, 9];
    let target = 5;
    let result = Solution::search_insert(nums, target);
    println!("目标值在数组中间: nums = [1,3,5,7,9], target = 5");
    println!("期望输出: 2, 实际输出: {}", result);
    println!();
    
    // 5. 目标值不存在，需要插入到不同位置
    let nums = vec![1, 3, 5, 7, 9];
    let target = 0;  // 插入到开头
    let result = Solution::search_insert(nums, target);
    println!("插入到开头: nums = [1,3,5,7,9], target = 0");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    let nums = vec![1, 3, 5, 7, 9];
    let target = 2;  // 插入到中间
    let result = Solution::search_insert(nums, target);
    println!("插入到中间: nums = [1,3,5,7,9], target = 2");
    println!("期望输出: 1, 实际输出: {}", result);
    println!();
    
    let nums = vec![1, 3, 5, 7, 9];
    let target = 10;  // 插入到末尾
    let result = Solution::search_insert(nums, target);
    println!("插入到末尾: nums = [1,3,5,7,9], target = 10");
    println!("期望输出: 5, 实际输出: {}", result);
    println!();
    
    // 6. 大数组测试
    let nums: Vec<i32> = (0..1000).step_by(2).collect();  // [0,2,4,6,...,998]
    let target = 500;
    let result = Solution::search_insert(nums, target);
    println!("大数组测试: nums = [0,2,4,...,998], target = 500");
    println!("期望输出: 250, 实际输出: {}", result);
    println!();
    
    // 7. 负数测试
    let nums = vec![-10, -5, 0, 5, 10];
    let target = -7;
    let result = Solution::search_insert(nums, target);
    println!("负数测试: nums = [-10,-5,0,5,10], target = -7");
    println!("期望输出: 1, 实际输出: {}", result);
    println!();
    
    // 8. 边界值测试
    let nums = vec![i32::MIN, -1000, 0, 1000, i32::MAX];
    let target = i32::MIN;
    let result = Solution::search_insert(nums, target);
    println!("最小整数测试: nums = [MIN,-1000,0,1000,MAX], target = MIN");
    println!("期望输出: 0, 实际输出: {}", result);
    println!();
    
    let nums = vec![i32::MIN, -1000, 0, 1000, i32::MAX];
    let target = i32::MAX;
    let result = Solution::search_insert(nums, target);
    println!("最大整数测试: nums = [MIN,-1000,0,1000,MAX], target = MAX");
    println!("期望输出: 4, 实际输出: {}", result);
    println!();
}