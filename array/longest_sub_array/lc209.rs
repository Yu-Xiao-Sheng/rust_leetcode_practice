/*
209. 长度最小的子数组
中等
相关标签
premium lock icon
相关企业
给定一个含有 n 个正整数的数组和一个正整数 target 。

找出该数组中满足其总和大于等于 target 的长度最小的 子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。

 

示例 1：

输入：target = 7, nums = [2,3,1,2,4,3]
输出：2
解释：子数组 [4,3] 是该条件下的长度最小的子数组。
示例 2：

输入：target = 4, nums = [1,4,4]
输出：1
示例 3：

输入：target = 11, nums = [1,1,1,1,1,1,1,1]
输出：0
 

提示：

1 <= target <= 109
1 <= nums.length <= 105
1 <= nums[i] <= 104
 */
struct Solution;

impl Solution {
    // 暴力解法 - O(n²) 时间复杂度
    #[allow(dead_code)]
    pub fn min_sub_array_len_brute_force(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum;
        let mut result = i32::MAX;
        for i in 0..nums.len() {
            sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum >= target {
                    let sub_len: i32 = (j - i + 1) as i32;
                    result = if result < sub_len { result } else { sub_len };
                    break;
                }
            }
        }

        return if result == i32::MAX { 0 } else { result };
    }

    // 滑动窗口解法 - O(n) 时间复杂度
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;  // 左指针
        let mut sum = 0;   // 当前窗口和
        let mut result = i32::MAX;  // 最小长度
        
        // 右指针遍历数组
        for right in 0..nums.len() {
            sum += nums[right];  // 扩大窗口
            
            // 当窗口和满足条件时，尝试缩小窗口
            while sum >= target {
                let window_len = (right - left + 1) as i32;
                result = result.min(window_len);  // 更新最小长度
                
                sum -= nums[left];  // 缩小窗口
                left += 1;
            }
        }
        
        if result == i32::MAX { 0 } else { result }
    }

    
}

fn main() {
    let target = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let result = Solution::min_sub_array_len(target, nums);
    println!("滑动窗口解法结果: {}", result);
    
    // 测试更多用例
    println!("测试用例1: target=4, nums=[1,4,4] -> {}", 
        Solution::min_sub_array_len2(4, vec![1,4,4]));
    println!("测试用例2: target=11, nums=[1,1,1,1,1,1,1,1] -> {}", 
        Solution::min_sub_array_len2(11, vec![1,1,1,1,1,1,1,1]));
    println!("测试用例3: target=15, nums=[1,2,3,4,5] -> {}", 
        Solution::min_sub_array_len2(15, vec![1,2,3,4,5]));
}