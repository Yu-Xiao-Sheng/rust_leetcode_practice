# 128. 最长连续序列（Rust 实现笔记）

## 算法思路（O(n)）
- 用哈希集合 `HashSet<i32>` 去重并支持 O(1) 查询。
- 仅从“序列起点”开始计数：当 `num - 1` 不在集合时，`num` 才可能是起点。
- 从起点不断检查 `num + 1, num + 2, ...` 是否在集合中，累加长度，更新最长值。
- 额外剪枝：若当前已达到集合元素数的一半，可提前返回（本实现保留这一点）。

## 关键 Rust 实现细节
1. 构建集合
   ```rust
   let set: HashSet<i32> = nums.into_iter().collect();
   if set.is_empty() { return 0; }
   ```

2. 判断起点并向右扩展
   ```rust
   for &num in &set {
       if set.contains(&(num - 1)) { continue; } // 不是起点，跳过
       let mut cur = num;
       let mut len = 1;
       while set.contains(&(cur + 1)) {
           cur += 1;
           len += 1;
       }
       best = best.max(len);
   }
   ```
   - 避免从非起点开始重复计数，保证线性复杂度。

3. 提前返回剪枝（可选）
   ```rust
   let half_len = (set.len() as i32 + 1) / 2;
   if best >= half_len { return best; }
   ```
   - 依题目特性可省略；存在时可更早结束。

4. 复杂度
   - 时间 O(n)：每个元素最多作为起点一次，内部扩展不会重复访问已处理的起点。
   - 空间 O(n)：存储哈希集合。

## 完整核心函数
```rust
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        if set.is_empty() {
            return 0;
        }

        let mut best = 0;
        let half_len = (set.len() as i32 + 1) / 2;

        for &num in &set {
            if set.contains(&(num - 1)) {
                continue;
            }
            let mut cur = num;
            let mut len = 1;
            while set.contains(&(cur + 1)) {
                cur += 1;
                len += 1;
            }
            best = best.max(len);
            if best >= half_len {
                return best;
            }
        }
        best
    }
}
```

## 常见坑
- 必须去重：原数组可有重复，直接遍历数组会重复计数。
- 从起点出发：若不检查 `num - 1`，可能导致 O(n^2)。
- 集合类型：使用 `HashSet<i32>`，避免借用生命周期问题，查询写法为 `set.contains(&value)`。
- 提前返回阈值非必需：只是微优化，保持/移除均不影响正确性。***
