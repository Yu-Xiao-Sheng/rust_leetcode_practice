/**

15. 三数之和
中等
相关标签
premium lock icon相关企业
提示

给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。

注意：答案中不可以包含重复的三元组。





示例 1：

输入：nums = [-1,0,1,2,-1,-4]
输出：[[-1,-1,2],[-1,0,1]]
解释：
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
注意，输出的顺序和三元组的顺序并不重要。

示例 2：

输入：nums = [0,1,1]
输出：[]
解释：唯一可能的三元组和不为 0 。

示例 3：

输入：nums = [0,0,0]
输出：[[0,0,0]]
解释：唯一可能的三元组和为 0 。


**/

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut nums2 = nums;
        nums2.sort_unstable();

        for i in 0..nums2.len() {
            let mut left = i + 1;
            let mut right = nums2.len() - 1;

            if nums2[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == num[i-1]{
                continue;
            }

            while left < right {
                let tem = nums2[left] + nums2[right] + nums2[i];
                if tem == 0 {
                    res.push(vec![nums2[i], nums2[left], nums2[right]]);
                } else if tem > 0 {
                    right -= 1;
                } else if tem < 0 {
                    left += 1;
                }
            }
        }

        res
    }
}

fn main() {
    // 调用Solution给出样例，并断言结果
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
}
