#!/usr/bin/env rust-script

/**
 * 26. 删除有序数组中的重复项
已解答
简单
相关标签
premium lock icon
相关企业
提示
给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。

考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过：

更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不重要。
返回 k 。
判题标准:

系统会用下面的代码来测试你的题解:

int[] nums = [...]; // 输入数组
int[] expectedNums = [...]; // 长度正确的期望答案

int k = removeDuplicates(nums); // 调用

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
如果所有断言都通过，那么您的题解将被 通过。



示例 1：

输入：nums = [1,1,2]
输出：2, nums = [1,2,_]
解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
示例 2：

输入：nums = [0,0,1,1,1,2,2,3,3,4]
输出：5, nums = [0,1,2,3,4]
解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。


提示：

1 <= nums.length <= 3 * 104
-104 <= nums[i] <= 104
nums 已按 非严格递增 排列
 */

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 特殊情况：如果数组为空，直接返回0
        if nums.is_empty() {
            return 0;
        }
        
        // 双指针法：
        // slow: 慢指针，指向当前不重复元素应该放置的位置
        // fast: 快指针，遍历整个数组寻找不重复元素
        let mut slow = 0;
        
        // 从第二个元素开始遍历（因为第一个元素肯定是不重复的）
        for fast in 1..nums.len() {
            // 如果快指针指向的元素与慢指针指向的元素不同
            // 说明找到了一个新的不重复元素
            if nums[fast] != nums[slow] {
                // 慢指针向前移动一位
                slow += 1;
                // 将新的不重复元素放到慢指针的位置
                nums[slow] = nums[fast];
            }
            // 如果 nums[fast] == nums[slow]，说明是重复元素
            // 快指针继续向前，慢指针不动
        }
        
        // 返回不重复元素的个数（slow + 1，因为索引从0开始）
        (slow + 1) as i32
    }
}

fn main() {
    // 测试用例1
    let mut nums1 = vec![1, 1, 2];
    println!("原数组: {:?}", nums1);
    let k1 = Solution::remove_duplicates(&mut nums1);
    println!("去重后长度: {}", k1);
    println!("去重后数组前{}个元素: {:?}", k1, &nums1[..k1 as usize]);
    println!();
    
    // 测试用例2
    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("原数组: {:?}", nums2);
    let k2 = Solution::remove_duplicates(&mut nums2);
    println!("去重后长度: {}", k2);
    println!("去重后数组前{}个元素: {:?}", k2, &nums2[..k2 as usize]);
    println!();
    
    // 你的测试用例
    let mut nums3 = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    println!("原数组: {:?}", nums3);
    let k3 = Solution::remove_duplicates(&mut nums3);
    println!("去重后长度: {}", k3);
    println!("去重后数组前{}个元素: {:?}", k3, &nums3[..k3 as usize]);
}

/*
算法步骤详解：

1. 初始状态：
   数组: [1, 1, 2, 2, 3, 3, 4, 4, 5, 5]
   slow: 0 (指向位置0，值为1)
   fast: 1 (从位置1开始遍历)

2. 遍历过程：
   fast=1: nums[1]=1, nums[0]=1, 相等，slow不动，fast继续
   fast=2: nums[2]=2, nums[0]=1, 不等，slow移到1，nums[1]=2
   fast=3: nums[3]=2, nums[1]=2, 相等，slow不动，fast继续  
   fast=4: nums[4]=3, nums[1]=2, 不等，slow移到2，nums[2]=3
   fast=5: nums[5]=3, nums[2]=3, 相等，slow不动，fast继续
   fast=6: nums[6]=4, nums[2]=3, 不等，slow移到3，nums[3]=4
   fast=7: nums[7]=4, nums[3]=4, 相等，slow不动，fast继续
   fast=8: nums[8]=5, nums[3]=4, 不等，slow移到4，nums[4]=5
   fast=9: nums[9]=5, nums[4]=5, 相等，slow不动，fast继续

3. 最终结果：
   数组变为: [1, 2, 3, 4, 5, 3, 4, 4, 5, 5] (前5个元素是去重后的结果)
   返回: slow + 1 = 4 + 1 = 5

核心思想：
- 慢指针维护已处理的不重复元素区域
- 快指针寻找新的不重复元素
- 只有当找到新元素时，才移动慢指针并更新元素
*/