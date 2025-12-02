# 283. 移动零（Rust 实现笔记）

## 算法思路
- 使用双指针：`fast` 遍历数组，`slow` 指向下一个应放置非零元素的位置。
- 遇到非零时，写到 `nums[slow]`，`slow += 1`。遍历完后，`slow..len` 全部填 0。
- 原地操作，时间 O(n)，空间 O(1)。

## 关键 Rust 写法
```rust
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }
    for i in slow..nums.len() {
        nums[i] = 0;
    }
}
```
- 传入 `&mut Vec<i32>` 原地修改；无返回值 `()`.
- 两段循环：第一段收集非零，第二段补零。

## 示例调用（原文件 `main`）
```rust
let mut nums1 = vec![0,1,0,3,12];
Solution::move_zeroes(&mut nums1);
assert_eq!(vec![1,3,12,0,0], nums1);
```
- 需先创建可变向量并传可变引用；断言比较修改后的向量。

## 常见坑
- 不要在 `assert_eq!` 中直接比较 `()`：函数返回值为空，需用修改后的 `Vec` 来比较。
- 注意覆盖写入：`nums[slow] = nums[fast]` 可以在同一数组内安全覆盖，因为 `slow <= fast`。
- 填零循环不能忘：若遗漏，尾部会残留旧值。***
