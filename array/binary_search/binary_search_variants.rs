// 查找第一个等于目标值的元素 - 左闭右开区间
fn binary_search_first_left_closed_right_open(nums: &[i32], target: i32) -> i32 {
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
    
    if left < nums.len() && nums[left] == target {
        left as i32
    } else {
        -1
    }
}

// 查找第一个等于目标值的元素 - 左闭右闭区间
fn binary_search_first_left_closed_right_closed(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            // 找到目标值，但需要找到第一个
            right = mid - 1;  // 继续向左找
        }
    }
    
    if left < nums.len() && nums[left] == target {
        left as i32
    } else {
        -1
    }
}

// 查找最后一个等于目标值的元素 - 左闭右开区间
fn binary_search_last_left_closed_right_open(nums: &[i32], target: i32) -> i32 {
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
    
    if left > 0 && nums[left - 1] == target {
        (left - 1) as i32
    } else {
        -1
    }
}

// 查找最后一个等于目标值的元素 - 左闭右闭区间
fn binary_search_last_left_closed_right_closed(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            // 找到目标值，但需要找到最后一个
            left = mid + 1;  // 继续向右找
        }
    }
    
    if right >= 0 && nums[right] == target {
        right as i32
    } else {
        -1
    }
}

fn main() {
    let nums = vec![1, 2, 2, 2, 3, 4, 5];
    let target = 2;
    
    println!("数组: {:?}", nums);
    println!("目标值: {}", target);
    println!();
    
    // 测试查找第一个
    let first_open = binary_search_first_left_closed_right_open(&nums, target);
    let first_closed = binary_search_first_left_closed_right_closed(&nums, target);
    println!("第一个等于 {} 的位置:", target);
    println!("  左闭右开: {}", first_open);
    println!("  左闭右闭: {}", first_closed);
    println!();
    
    // 测试查找最后一个
    let last_open = binary_search_last_left_closed_right_open(&nums, target);
    let last_closed = binary_search_last_left_closed_right_closed(&nums, target);
    println!("最后一个等于 {} 的位置:", target);
    println!("  左闭右开: {}", last_open);
    println!("  左闭右闭: {}", last_closed);
    println!();
    
    // 验证结果
    println!("验证:");
    println!("  第一个2在位置{}: {}", first_open, nums[first_open as usize]);
    println!("  最后一个2在位置{}: {}", last_open, nums[last_open as usize]);
} 