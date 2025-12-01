use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    // 记录已访问数字的下标，方便 O(1) 查询
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, &v) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - v)) {
            return vec![i as i32, j as i32];
        }
        map.insert(v, i);
    }
    vec![]
}

fn main() {
    let arr = vec![2, 11, 7, 15];
    let target = 9;
    let res = two_sum(&arr, target);
    println!("{:?}", res);
}
