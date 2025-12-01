# 49. 字母异位词分组（Rust 实现要点）

## 算法思路
- 将每个字符串按字符排序，排序后的结果作为「分组键」，原字符串作为值存入同一个列表。
- 最终返回哈希表中所有的值列表即可。

## 关键 Rust 细节
1. 将 `String` 拆成字符并排序
   ```rust
   let mut chars: Vec<char> = s.chars().collect(); // String -> Vec<char>
   chars.sort_unstable(); // 原地排序，字母异位词会得到相同的序列
   let key: String = chars.into_iter().collect(); // Vec<char> -> String
   ```
   - `chars()` 按 Unicode 迭代；题目只有小写字母，不会有复杂排序问题。
   - `sort_unstable` 比 `sort` 更快且足够安全，因为只需一致性不需稳定性。

2. 使用 `HashMap` 聚合
   ```rust
   use std::collections::HashMap;
   let mut groups: HashMap<String, Vec<String>> = HashMap::new();

   // 将排序后的 key 作为字典键，原字符串作为值 push 进去
   groups.entry(key).or_default().push(s);
   ```
   - `HashMap<String, Vec<String>>`：键和值都拥有所有权，方便存储和移动。
   - `entry(key).or_default()`：如果键不存在则插入默认的空 `Vec`，然后返回可变引用，直接 `push`。
   - 若想手动写，可用：
     ```rust
     groups.insert(key.clone(), vec![s]);
     // 或者先 get_mut，再 push：
     if let Some(bucket) = groups.get_mut(&key) { bucket.push(s); }
     ```

3. 从哈希表取出最终结果
   ```rust
   let result: Vec<Vec<String>> = groups.into_values().collect();
   ```
   - `into_values()` 直接拿走所有值（也就是各分组的字符串列表），避免再遍历键。

## 完整核心函数
```rust
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key: String = chars.into_iter().collect();
            groups.entry(key).or_default().push(s);
        }

        groups.into_values().collect()
    }
}
```

## 常见坑
- `String` 转字符排序：不能直接在 `String` 上排序，需要先收集为 `Vec<char>`。
- 键类型需拥有所有权：用 `String` 而不是 `&str`，避免借用生命周期问题。
- 推入哈希表时避免多次查找：用 `entry().or_default()` 一步到位。
- 输出顺序不定：哈希表无序，如需展示稳定结果可额外排序副本。***
