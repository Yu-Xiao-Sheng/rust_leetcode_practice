# 11. 盛最多水的容器（Rust 实现笔记）

## 算法思路（双指针，O(n)）
- 夹逼法：左右指针从两端出发，每次以较短的一侧为瓶颈计算面积，并向内收缩那一侧。
- 逻辑：面积由短板决定，移动长板无效；只有移动短板才可能得到更高的短板从而提升面积。
- 过程：`area = min(height[l], height[r]) * (r - l)`，更新最大值，若 `height[l] < height[r]` 则 `l += 1` 否则 `r -= 1`。

## 关键 Rust 写法
```rust
use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0usize;
    let mut r = height.len() - 1;
    let mut best = 0;

    while l < r {
        let h = min(height[l], height[r]);
        best = max(best, h * (r - l) as i32);
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    best
}
```
- 指针用 `usize`；计算宽度 `(r - l) as i32`。
- `min`/`max` 来更新当前面积和最大值。

## 示例
```rust
assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
```

## 常见坑
- 注意索引类型：使用 `usize` 避免负数，宽度转换为 `i32` 再与高度相乘。
- 每次只移动短板：移动长板不会增加有效高度，避免无谓计算。***
