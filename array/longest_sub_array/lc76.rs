/*
76. 最小覆盖子串
困难
相关标签
premium lock icon
相关企业
提示
给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。



注意：

对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
如果 s 中存在这样的子串，我们保证它是唯一的答案。


示例 1：

输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"
解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
示例 2：

输入：s = "a", t = "a"
输出："a"
解释：整个字符串 s 是最小覆盖子串。
示例 3:

输入: s = "a", t = "aa"
输出: ""
解释: t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。


提示：

m == s.length
n == t.length
1 <= m, n <= 105
s 和 t 由英文字母组成

*/

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;

        // 1. 统计目标字符串中每个字符的个数
        let mut need_count = HashMap::new();
        for ch in t.chars() {
            *need_count.entry(ch).or_insert(0) += 1;
        }

        // 2. 初始化滑动窗口相关变量
        let mut window_count = HashMap::new();  // 当前窗口内各字符的个数
        let mut left = 0;                      // 左指针
        let mut valid = 0;                     // 已经满足条件的字符种类数（修复拼写错误）

        let mut min_len = i32::MAX;       // 最小窗口长度
        let mut start = 0;                     // 最小窗口的起始位置

        // 3. 将字符串转换为字符数组，避免字节索引问题
        let s_chars: Vec<char> = s.chars().collect();

        // 4. 滑动窗口算法
        for right in 0..s_chars.len() {
            let ch = s_chars[right];
            
            // 扩大窗口：将右指针指向的字符加入窗口
            if need_count.contains_key(&ch) {
                *window_count.entry(ch).or_insert(0) += 1;
                
                // 如果某个字符的数量刚好满足要求，valid计数+1
                if window_count[&ch] == need_count[&ch] {
                    valid += 1;
                }
            }

            // 收缩窗口：当窗口满足条件时，尝试缩小窗口
            while valid == need_count.len() {
                // 更新最小长度（修复：窗口长度应该是 right - left + 1）
                if right - left + 1 < min_len as usize {
                    start = left;
                    min_len = (right - left + 1) as i32;
                }

                // 移动左指针，缩小窗口
                let left_ch = s_chars[left];  // 修复：使用字符数组而不是字节数组
                left += 1;

                // 更新窗口内字符计数
                if need_count.contains_key(&left_ch) {
                    // 如果移除的字符刚好满足要求，valid计数-1
                    if window_count[&left_ch] == need_count[&left_ch] {
                        valid -= 1;
                    }
                    *window_count.get_mut(&left_ch).unwrap() -= 1;
                }
            }
        }

        // 5. 返回结果
        if min_len == i32::MAX {
            String::new()  // 没有找到符合条件的子串
        } else {
            // 修复：使用字符数组切片，避免字节索引问题
            s_chars[start..start + min_len as usize].iter().collect()
        }
    }
}

fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    let result = Solution::min_window(s, t);
    println!("result: {}", result);
}
