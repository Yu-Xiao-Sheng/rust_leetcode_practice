// 查找有序列表中第一个等于目标值的下标
fn find_first_equals(list: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if list[mid] < target {
            left = mid + 1;
        } else {
            right = mid; //即使相等，也继续向左寻找
        }
    }

    if left < list.len() && list[left] == target {
        return left as i32;
    }
    -1
}

// 查找有序列表中，最后一个等于目标值的下标
fn find_last_equals(list: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if list[mid] <= target {
            left = mid + 1; // 即使相等也继续向右找
        }else{
            right = mid;
        }
    }

    if left > 0 && list[left - 1] == target {
        (left - 1) as i32
    } else {
        -1
    }
}

fn main() {
    let list = vec![-7, -5, -1, 0, 0, 0, 2, 3, 3, 3, 5, 6, 8, 9, 9, 9];
    let target = 3;

    let result = find_first_equals(&list, target);
    println!("result: {}", result);
}
