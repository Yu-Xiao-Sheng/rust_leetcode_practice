// 使用公共测试验证器的示例

// 引入测试验证器模块（从上层目录）
mod test_validator {
    include!("../test_validator.rs");
}
use test_validator::{TestValidator, TestGroup, TestSuite};

// 示例：LeetCode 35 搜索插入位置
struct Solution35;

impl Solution35 {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}

// 示例：LeetCode 34 在排序数组中查找元素的第一个和最后一个位置
struct Solution34;

impl Solution34 {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut left = 0;
        let mut right = nums.len();
        
        // 找第一个等于目标值的位置
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        if left < nums.len() && nums[left] == target {
            result.push(left as i32);
        } else {
            result.push(-1);
        }
        
        // 找最后一个等于目标值的位置
        left = 0;
        right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        if left > 0 && nums[left - 1] == target {
            result.push((left - 1) as i32);
        } else {
            result.push(-1);
        }
        
        result
    }
}

fn main() {
    // 示例1：使用 TestValidator 进行简单测试
    println!("=== 简单测试示例 ===");
    let mut validator = TestValidator::new();
    
    test_case_int!(validator, "简单加法", 2 + 2, 4);
    test_case_bool!(validator, "简单比较", 5 > 3, true);
    test_case_vec!(validator, "向量测试", &vec![1, 2, 3], &vec![1, 2, 3]);
    
    validator.print_summary();
    
    // 示例2：使用 TestGroup 进行分组测试
    println!("\n=== 分组测试示例 ===");
    let mut group = TestGroup::new("LeetCode 35 测试");
    
    test_case_int!(group.validator(), "示例1", Solution35::search_insert(vec![1,3,5,6], 5), 2);
    test_case_int!(group.validator(), "示例2", Solution35::search_insert(vec![1,3,5,6], 2), 1);
    test_case_int!(group.validator(), "示例3", Solution35::search_insert(vec![1,3,5,6], 7), 4);
    
    group.print_summary();
    
    // 示例3：使用 TestSuite 进行完整测试套件
    println!("\n=== 完整测试套件示例 ===");
    let mut test_suite = TestSuite::new("LeetCode 34 测试套件");
    
    // 测试组1：题目示例
    let mut group = TestGroup::new("题目示例测试");
    test_case_vec!(group.validator(), "示例1", &Solution34::search_range(vec![5,7,7,8,8,10], 8), &vec![3,4]);
    test_case_vec!(group.validator(), "示例2", &Solution34::search_range(vec![5,7,7,8,8,10], 6), &vec![-1,-1]);
    test_case_vec!(group.validator(), "示例3", &Solution34::search_range(vec![], 0), &vec![-1,-1]);
    test_suite.add_group(group);
    
    // 测试组2：边界情况
    let mut group = TestGroup::new("边界情况测试");
    test_case_vec!(group.validator(), "单个元素且等于目标值", &Solution34::search_range(vec![5], 5), &vec![0,0]);
    test_case_vec!(group.validator(), "单个元素且不等于目标值", &Solution34::search_range(vec![5], 3), &vec![-1,-1]);
    test_case_vec!(group.validator(), "两个相同元素", &Solution34::search_range(vec![2,2], 2), &vec![0,1]);
    test_suite.add_group(group);
    
    // 测试组3：多个相同元素
    let mut group = TestGroup::new("多个相同元素测试");
    test_case_vec!(group.validator(), "所有元素都相同", &Solution34::search_range(vec![2,2,2,2,2], 2), &vec![0,4]);
    test_case_vec!(group.validator(), "中间有多个相同元素", &Solution34::search_range(vec![1,2,2,2,3], 2), &vec![1,3]);
    test_suite.add_group(group);
    
    test_suite.print_final_summary();
    
    // 示例4：使用不同的断言方法
    println!("\n=== 不同断言方法示例 ===");
    let mut validator = TestValidator::new();
    
    // 使用通用断言
    validator.assert_eq("字符串比较", "hello", "hello");
    validator.assert_eq_debug("向量比较", &vec![1, 2, 3], &vec![1, 2, 3]);
    validator.assert_float("浮点数比较", 3.14159, 3.14159, 0.00001);
    validator.assert_string("字符串断言", "test", "test");
    
    validator.print_summary();
} 