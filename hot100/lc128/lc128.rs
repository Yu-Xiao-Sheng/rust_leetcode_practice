use std::collections::HashSet;

/**
128. 最长连续序列
中等
相关标签
premium lock icon相关企业

给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。

请你设计并实现时间复杂度为 O(n) 的算法解决此问题。



示例 1：

输入：nums = [100,4,200,1,3,2]
输出：4
解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。

示例 2：

输入：nums = [0,3,7,2,5,8,4,6,0,1]
输出：9

示例 3：

输入：nums = [1,0,1,2]
输出：3



提示：

    0 <= nums.length <= 105
    -109 <= nums[i] <= 109


**/


struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 先将 nums 转为集合，方便 O(1) 判断是否存在。
        let set: HashSet<i32> = nums.into_iter().collect();
        if set.is_empty() {
            return 0;
        }

        let mut best = 0;
        let half_len = (set.len() as i32 + 1) / 2; // 提前返回的阈值

        for &num in &set {
            // 只有当 num 没有前驱时，才从它开始向后数，避免重复计算。
            if set.contains(&(num - 1)) {
                continue;
            }

            let mut cur = num;
            let mut len = 1;
            while set.contains(&(cur + 1)) {
                cur += 1;
                len += 1;
            }

            if len > best {
                best = len;
                if best >= half_len {
                    return best;
                }
            }
        }

        best
    }
}

fn main() {
    let samples = vec![
        vec![100, 4, 200, 1, 3, 2],
        vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
        vec![1, 0, 1, 2],
    ];

    for sample in samples {
        let ans = Solution::longest_consecutive(sample.clone());
        println!("输入: {:?} -> 最长连续序列长度: {}", sample, ans);
    }
}
