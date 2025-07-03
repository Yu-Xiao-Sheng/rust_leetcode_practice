/*
滑动窗口经典变体题目合集
========================

滑动窗口主要分为两大类：
1. 固定窗口大小
2. 可变窗口大小 (又分为求最大/最小)

本文件包含最经典的滑动窗口变体题目
*/

struct Solution;

impl Solution {
    // ========== 变体1: LeetCode 3 - 无重复字符的最长子串 ==========
    // 题目：给定一个字符串 s ，请你找出其中不含有重复字符的最长子串的长度
    // 类型：可变窗口 - 求最大长度
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        
        let chars: Vec<char> = s.chars().collect();
        let mut char_index = HashMap::new();  // 字符 -> 最新出现位置
        let mut left = 0;
        let mut max_len = 0;
        
        for right in 0..chars.len() {
            let current_char = chars[right];
            
            // 如果字符重复，移动左指针
            if let Some(&last_index) = char_index.get(&current_char) {
                left = left.max(last_index + 1);
            }
            
            char_index.insert(current_char, right);
            max_len = max_len.max(right - left + 1);
        }
        
        max_len as i32
    }
    
    // ========== 变体2: LeetCode 76 - 最小覆盖子串 ==========
    // 题目：给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串
    // 类型：可变窗口 - 求最小长度
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        
        if s.len() < t.len() { return String::new(); }
        
        let s_chars: Vec<char> = s.chars().collect();
        let mut target_count = HashMap::new();
        let mut window_count = HashMap::new();
        
        // 统计目标字符串中每个字符的个数
        for ch in t.chars() {
            *target_count.entry(ch).or_insert(0) += 1;
        }
        
        let mut left = 0;
        let mut min_len = usize::MAX;
        let mut min_start = 0;
        let mut formed = 0;  // 已经满足条件的字符种类数
        let required = target_count.len();
        
        for right in 0..s_chars.len() {
            let ch = s_chars[right];
            *window_count.entry(ch).or_insert(0) += 1;
            
            if target_count.contains_key(&ch) && 
               window_count[&ch] == target_count[&ch] {
                formed += 1;
            }
            
            // 尝试收缩窗口
            while formed == required && left <= right {
                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    min_start = left;
                }
                
                let left_ch = s_chars[left];
                *window_count.get_mut(&left_ch).unwrap() -= 1;
                
                if target_count.contains_key(&left_ch) && 
                   window_count[&left_ch] < target_count[&left_ch] {
                    formed -= 1;
                }
                
                left += 1;
            }
        }
        
        if min_len == usize::MAX {
            String::new()
        } else {
            s_chars[min_start..min_start + min_len].iter().collect()
        }
    }
    
    // ========== 变体3: LeetCode 438 - 找到字符串中所有字母异位词 ==========
    // 题目：给定两个字符串 s 和 p，找到 s 中所有 p 的异位词的子串
    // 类型：固定窗口大小
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() { return vec![]; }
        
        let s_chars: Vec<char> = s.chars().collect();
        let p_len = p.len();
        let mut result = vec![];
        
        // 统计目标字符串的字符频次
        let mut p_count = [0; 26];
        for ch in p.chars() {
            p_count[(ch as u8 - b'a') as usize] += 1;
        }
        
        // 滑动窗口
        let mut window_count = [0; 26];
        
        for i in 0..s_chars.len() {
            // 扩大窗口
            let right_char = s_chars[i];
            window_count[(right_char as u8 - b'a') as usize] += 1;
            
            // 如果窗口大小超过目标长度，缩小窗口
            if i >= p_len {
                let left_char = s_chars[i - p_len];
                window_count[(left_char as u8 - b'a') as usize] -= 1;
            }
            
            // 检查是否匹配
            if i >= p_len - 1 && window_count == p_count {
                result.push((i - p_len + 1) as i32);
            }
        }
        
        result
    }
    
    // ========== 变体4: LeetCode 904 - 水果成篮 ==========
    // 题目：你有两个篮子，每个篮子可以携带任何数量的水果，但每个篮子只能携带一种类型的水果
    // 类型：可变窗口 - 最多包含2种不同元素的最长子数组
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        
        let mut fruit_count = HashMap::new();
        let mut left = 0;
        let mut max_fruits = 0;
        
        for right in 0..fruits.len() {
            // 扩大窗口
            *fruit_count.entry(fruits[right]).or_insert(0) += 1;
            
            // 如果水果种类超过2种，缩小窗口
            while fruit_count.len() > 2 {
                let left_fruit = fruits[left];
                *fruit_count.get_mut(&left_fruit).unwrap() -= 1;
                
                if fruit_count[&left_fruit] == 0 {
                    fruit_count.remove(&left_fruit);
                }
                
                left += 1;
            }
            
            max_fruits = max_fruits.max(right - left + 1);
        }
        
        max_fruits as i32
    }
    
    // ========== 变体5: LeetCode 239 - 滑动窗口最大值 ==========
    // 题目：给你一个整数数组 nums，有一个大小为 k 的滑动窗口
    // 类型：固定窗口大小 - 维护窗口最大值
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        
        let k = k as usize;
        let mut deque: VecDeque<usize> = VecDeque::new();  // 存储索引
        let mut result = vec![];
        
        for i in 0..nums.len() {
            // 移除超出窗口范围的索引
            while let Some(&front) = deque.front() {
                if front <= i.saturating_sub(k) {
                    deque.pop_front();
                } else {
                    break;
                }
            }
            
            // 维护递减队列
            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            
            deque.push_back(i);
            
            // 如果窗口大小达到k，记录最大值
            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }
        
        result
    }
}

fn main() {
    println!("🚀 滑动窗口变体题目测试");
    
    // 测试无重复字符最长子串
    let s1 = "abcabcbb".to_string();
    println!("\n1. 无重复字符最长子串:");
    println!("输入: '{}' -> 输出: {}", s1, Solution::length_of_longest_substring(s1));
    
    // 测试最小覆盖子串
    let s2 = "ADOBECODEBANC".to_string();
    let t2 = "ABC".to_string();
    println!("\n2. 最小覆盖子串:");
    println!("s: '{}', t: '{}' -> 输出: '{}'", s2, t2, Solution::min_window(s2, t2));
    
    // 测试字母异位词
    let s3 = "cbaebabacd".to_string();
    let p3 = "abc".to_string();
    println!("\n3. 找字母异位词:");
    println!("s: '{}', p: '{}' -> 输出: {:?}", s3, p3, Solution::find_anagrams(s3, p3));
    
    // 测试水果成篮
    let fruits = vec![1, 2, 1];
    println!("\n4. 水果成篮:");
    println!("fruits: {:?} -> 输出: {}", fruits, Solution::total_fruit(fruits));
    
    // 测试滑动窗口最大值
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    println!("\n5. 滑动窗口最大值:");
    println!("nums: {:?}, k: {} -> 输出: {:?}", nums, k, Solution::max_sliding_window(nums, k));
} 