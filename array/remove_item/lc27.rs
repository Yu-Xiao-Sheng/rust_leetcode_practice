/**
 * 27. 移除元素
简单
相关标签
premium lock icon
相关企业
提示
给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素。元素的顺序可能发生改变。然后返回 nums 中与 val 不同的元素的数量。

假设 nums 中不等于 val 的元素数量为 k，要通过此题，您需要执行以下操作：

更改 nums 数组，使 nums 的前 k 个元素包含不等于 val 的元素。nums 的其余元素和 nums 的大小并不重要。
返回 k。
用户评测：

评测机将使用以下代码测试您的解决方案：

int[] nums = [...]; // 输入数组
int val = ...; // 要移除的值
int[] expectedNums = [...]; // 长度正确的预期答案。
                            // 它以不等于 val 的值排序。

int k = removeElement(nums, val); // 调用你的实现

assert k == expectedNums.length;
sort(nums, 0, k); // 排序 nums 的前 k 个元素
for (int i = 0; i < actualLength; i++) {
    assert nums[i] == expectedNums[i];
}
如果所有的断言都通过，你的解决方案将会 通过。

 

示例 1：

输入：nums = [3,2,2,3], val = 3
输出：2, nums = [2,2,_,_]
解释：你的函数函数应该返回 k = 2, 并且 nums 中的前两个元素均为 2。
你在返回的 k 个元素之外留下了什么并不重要（因此它们并不计入评测）。
示例 2：

输入：nums = [0,1,2,2,3,0,4,2], val = 2
输出：5, nums = [0,1,4,0,3,_,_,_]
解释：你的函数应该返回 k = 5，并且 nums 中的前五个元素为 0,0,1,3,4。
注意这五个元素可以任意顺序返回。
你在返回的 k 个元素之外留下了什么并不重要（因此它们并不计入评测）。
 

提示：

0 <= nums.length <= 100
0 <= nums[i] <= 50
0 <= val <= 100
 */

use rand::{thread_rng, Rng};
use std::collections::HashSet;

// 引入测试验证器
mod test_validator {
    include!("../../test_validator.rs");
}
use test_validator::*;

struct Solution;

impl Solution {
    // 双指针法 - 快慢指针
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow_index = 0; // 慢指针，记录不等于val的元素位置
        
        for fast_index in 0..nums.len() {
            if nums[fast_index] != val {
                nums[slow_index] = nums[fast_index];
                slow_index += 1;
            }
        }
        
        slow_index as i32
    }
    
    // 对数器 - 暴力解法（用于验证正确性）
    pub fn remove_element_brute_force(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut result = Vec::new();
        
        // 收集所有不等于val的元素
        for &num in nums.iter() {
            if num != val {
                result.push(num);
            }
        }
        
        // 将结果写回原数组的前k个位置
        let k = result.len();
        for i in 0..k {
            nums[i] = result[i];
        }
        
        k as i32
    }
}

// 生成随机测试用例
fn generate_random_test_case() -> (Vec<i32>, i32) {
    let mut rng = thread_rng();
    
    // 随机数组长度 (0-100)
    let len = rng.gen_range(0..=100);
    
    // 随机数组元素 (0-50)
    let mut nums = Vec::new();
    for _ in 0..len {
        nums.push(rng.gen_range(0..=50));
    }
    
    // 随机val值 (0-100)
    let val = rng.gen_range(0..=100);
    
    (nums, val)
}

// 验证结果是否正确
fn validate_result(original_nums: &[i32], result_nums: &[i32], k: i32, val: i32) -> bool {
    let k = k as usize;
    
    // 检查返回的k值是否正确
    let expected_k = original_nums.iter().filter(|&&x| x != val).count();
    if k != expected_k {
        return false;
    }
    
    // 检查前k个元素是否都不等于val
    for i in 0..k {
        if result_nums[i] == val {
            return false;
        }
    }
    
    // 检查前k个元素是否包含了原数组中所有不等于val的元素
    let mut original_non_val: Vec<i32> = original_nums.iter()
        .filter(|&&x| x != val)
        .cloned()
        .collect();
    let mut result_first_k: Vec<i32> = result_nums[0..k].to_vec();
    
    original_non_val.sort();
    result_first_k.sort();
    
    original_non_val == result_first_k
}

fn main() {
    let mut test_suite = TestSuite::new("LeetCode 27 - 移除元素");
    
    // === 基本功能测试 ===
    {
        let mut group = TestGroup::new("基本功能测试");
        let validator = group.validator();
        
        // 示例1测试
        {
            let mut nums = vec![3, 2, 2, 3];
            let original = nums.clone();
            let val = 3;
            let k = Solution::remove_element(&mut nums, val);
            
            validator.assert_int("示例1 - 返回值", k, 2);
            
            // 验证前k个元素
            let mut result_first_k = nums[0..k as usize].to_vec();
            result_first_k.sort();
            validator.assert_vec("示例1 - 前k个元素", &result_first_k, &[2, 2]);
        }
        
        // 示例2测试
        {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            let original = nums.clone();
            let val = 2;
            let k = Solution::remove_element(&mut nums, val);
            
            validator.assert_int("示例2 - 返回值", k, 5);
            
            // 验证前k个元素
            let mut result_first_k = nums[0..k as usize].to_vec();
            result_first_k.sort();
            validator.assert_vec("示例2 - 前k个元素", &result_first_k, &[0, 0, 1, 3, 4]);
        }
        
        group.print_summary();
        test_suite.add_group(group);
    }
    
    // === 边界测试 ===
    {
        let mut group = TestGroup::new("边界测试");
        let validator = group.validator();
        
        // 空数组
        {
            let mut nums = vec![];
            let k = Solution::remove_element(&mut nums, 1);
            validator.assert_int("空数组", k, 0);
        }
        
        // 单元素数组 - 需要移除
        {
            let mut nums = vec![1];
            let k = Solution::remove_element(&mut nums, 1);
            validator.assert_int("单元素数组-需要移除", k, 0);
        }
        
        // 单元素数组 - 不需要移除
        {
            let mut nums = vec![1];
            let k = Solution::remove_element(&mut nums, 2);
            validator.assert_int("单元素数组-不需要移除", k, 1);
            validator.assert_int("单元素数组-元素值", nums[0], 1);
        }
        
        // 所有元素都需要移除
        {
            let mut nums = vec![2, 2, 2, 2];
            let k = Solution::remove_element(&mut nums, 2);
            validator.assert_int("所有元素都移除", k, 0);
        }
        
        // 所有元素都不需要移除
        {
            let mut nums = vec![1, 3, 4, 5];
            let original = nums.clone();
            let k = Solution::remove_element(&mut nums, 2);
            validator.assert_int("所有元素都保留-返回值", k, 4);
            validator.assert_vec("所有元素都保留-数组内容", &nums[0..k as usize], &original);
        }
        
        // 最大长度数组 (100个元素)
        {
            let mut nums: Vec<i32> = (0..100).map(|i| i % 10).collect();
            let original = nums.clone();
            let val = 5;
            let k = Solution::remove_element(&mut nums, val);
            
            let expected_k = original.iter().filter(|&&x| x != val).count() as i32;
            validator.assert_int("最大长度数组-返回值", k, expected_k);
            
            // 验证前k个元素都不等于val
            let all_valid = nums[0..k as usize].iter().all(|&x| x != val);
            validator.assert_bool("最大长度数组-元素有效性", all_valid, true);
        }
        
        // 边界值测试
        {
            let mut nums = vec![0, 50, 0, 50];
            let k1 = Solution::remove_element(&mut nums, 0);
            validator.assert_int("边界值-移除最小值", k1, 2);
            
            let mut nums = vec![0, 50, 0, 50];
            let k2 = Solution::remove_element(&mut nums, 50);
            validator.assert_int("边界值-移除最大值", k2, 2);
            
            let mut nums = vec![0, 50, 0, 50];
            let k3 = Solution::remove_element(&mut nums, 100);
            validator.assert_int("边界值-移除超出范围值", k3, 4);
        }
        
        group.print_summary();
        test_suite.add_group(group);
    }
    
    // === 对数器测试 ===
    {
        let mut group = TestGroup::new("对数器测试");
        let validator = group.validator();
        
        println!("开始对数器测试，运行1000次随机测试用例...");
        
        let mut all_passed = true;
        for i in 0..1000 {
            let (original_nums, val) = generate_random_test_case();
            
            // 准备两份相同的数据
            let mut nums1 = original_nums.clone();
            let mut nums2 = original_nums.clone();
            
            // 分别用两种方法求解
            let k1 = Solution::remove_element(&mut nums1, val);
            let k2 = Solution::remove_element_brute_force(&mut nums2, val);
            
            // 验证结果
            if k1 != k2 {
                println!("❌ 对数器测试失败 #{}: 返回值不同", i + 1);
                println!("   输入: {:?}, val: {}", original_nums, val);
                println!("   双指针法返回: {}, 暴力法返回: {}", k1, k2);
                all_passed = false;
                break;
            }
            
            // 验证双指针法的结果是否正确
            if !validate_result(&original_nums, &nums1, k1, val) {
                println!("❌ 对数器测试失败 #{}: 双指针法结果错误", i + 1);
                println!("   输入: {:?}, val: {}", original_nums, val);
                println!("   结果: {:?}, k: {}", nums1, k1);
                all_passed = false;
                break;
            }
            
            // 验证暴力法的结果是否正确
            if !validate_result(&original_nums, &nums2, k2, val) {
                println!("❌ 对数器测试失败 #{}: 暴力法结果错误", i + 1);
                println!("   输入: {:?}, val: {}", original_nums, val);
                println!("   结果: {:?}, k: {}", nums2, k2);
                all_passed = false;
                break;
            }
            
            if (i + 1) % 100 == 0 {
                println!("已完成 {} 次测试", i + 1);
            }
        }
        
        validator.assert_bool("1000次随机测试", all_passed, true);
        
        group.print_summary();
        test_suite.add_group(group);
    }
    
    // === 特殊模式测试 ===
    {
        let mut group = TestGroup::new("特殊模式测试");
        let validator = group.validator();
        
        // 交替模式
        {
            let mut nums = vec![1, 2, 1, 2, 1, 2];
            let k = Solution::remove_element(&mut nums, 1);
            validator.assert_int("交替模式-移除1", k, 3);
            let mut result = nums[0..k as usize].to_vec();
            result.sort();
            validator.assert_vec("交替模式-结果", &result, &[2, 2, 2]);
        }
        
        // 连续相同元素
        {
            let mut nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
            let k = Solution::remove_element(&mut nums, 2);
            validator.assert_int("连续相同元素", k, 6);
            let mut result = nums[0..k as usize].to_vec();
            result.sort();
            validator.assert_vec("连续相同元素-结果", &result, &[1, 1, 1, 3, 3, 3]);
        }
        
        // 头尾都是目标值
        {
            let mut nums = vec![5, 1, 2, 3, 5];
            let k = Solution::remove_element(&mut nums, 5);
            validator.assert_int("头尾都是目标值", k, 3);
            let mut result = nums[0..k as usize].to_vec();
            result.sort();
            validator.assert_vec("头尾都是目标值-结果", &result, &[1, 2, 3]);
        }
        
        group.print_summary();
        test_suite.add_group(group);
    }
    
    // 打印最终统计
    test_suite.print_final_summary();
}