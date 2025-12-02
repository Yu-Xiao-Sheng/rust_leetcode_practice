/**

283. 移动零
简单
相关标签
premium lock icon相关企业
提示

给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

请注意 ，必须在不复制数组的情况下原地对数组进行操作。



示例 1:

输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]

示例 2:

输入: nums = [0]
输出: [0]



提示:

    1 <= nums.length <= 104
    -231 <= nums[i] <= 231 - 1



进阶：你能尽量减少完成的操作次数吗？


**/

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len(){
            if nums[j] != 0{
                nums[i] = nums[j];
                i+=1;
            }
            j+=1;
        }

        while i < nums.len(){
            nums[i] = 0;
            i+=1;
        }
    }
}

fn main() {
    let mut nums1 = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut nums1);
    assert_eq!(vec![1,3,12,0,0], nums1);

    let mut nums2 = vec![0];
    Solution::move_zeroes(&mut nums2);
    assert_eq!(vec![0], nums2);
}
