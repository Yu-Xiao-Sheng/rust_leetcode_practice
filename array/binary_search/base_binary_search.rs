// 基础二分查找左闭右闭写法
fn binary_search_with_equals(nums: &[i32], target: i32) -> i32 {
    // 定义为作闭右闭区间
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right-left) / 2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid - 1;
        } else {
            return mid as i32;
        }
    }
    -1
}


// 基础二分查找左闭右开写法
fn binary_search_with_not_equals(nums: &[i32], target: i32) -> i32 {
    // 定义变为左闭右开区间
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right-left)/2;
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid;  // 左闭右开区间，right = mid
        } else {
            return mid as i32;  // 需要 return 关键字
        }
    }
    -1
}

fn main() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;

    let result = binary_search_with_equals(&nums, target);
    println!("result: {}", result);
}


