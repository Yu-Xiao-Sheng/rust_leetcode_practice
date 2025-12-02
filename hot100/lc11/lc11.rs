use std::cmp::{max, min};

/**
11. 盛最多水的容器
中等
相关标签
premium lock icon相关企业
提示

给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。

找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

返回容器可以储存的最大水量。

说明：你不能倾斜容器。



示例 1：

输入：[1,8,6,2,5,4,8,3,7]
输出：49
解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。

示例 2：

输入：height = [1,1]
输出：1



提示：

    n == height.length
    2 <= n <= 105
    0 <= height[i] <= 104


**/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0 as usize;
        let mut j = height.len() - 1;
        let mut max_area = 0;
        while i < j {
            let t = min(height[i],height[j]);
            max_area = max(max_area, t * (j-i) as i32);
            if height[i] < height[j] {
                i+=1;
            }else{
                j-=1;
            }
        }
        max_area
    }
}

fn main() {

    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);

}