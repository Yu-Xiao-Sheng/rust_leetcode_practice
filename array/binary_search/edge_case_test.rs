// 查找有序列表中，最后一个等于目标值的下标
fn find_last_equals(list: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if list[mid] <= target {
            left = mid + 1; // 即使相等也继续向右找
        } else {
            right = mid;
        }
    }

    // 关键：left 可能等于 0！
    if left > 0 && list[left - 1] == target {
        (left - 1) as i32
    } else {
        -1
    }
}

fn main() {
    // 测试边界情况
    let test_cases = vec![
        (vec![1, 2, 2, 2, 3], 2),      // 正常情况
        (vec![2, 2, 2, 2, 2], 2),      // 全是目标值
        (vec![1, 1, 1, 1, 1], 2),      // 没有目标值
        (vec![2], 2),                  // 单个元素且等于目标值
        (vec![1], 2),                  // 单个元素但不等于目标值
        (vec![], 2),                   // 空数组
    ];

    for (nums, target) in test_cases {
        let result = find_last_equals(&nums, target);
        println!("数组: {:?}, 目标: {}, 结果: {}", nums, target, result);
        
        // 验证 left 的值
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        println!("  循环结束后的 left: {}", left);
        println!();
    }
} 