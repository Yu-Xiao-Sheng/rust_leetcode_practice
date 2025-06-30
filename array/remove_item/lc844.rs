#!/usr/bin/env rust-script

/*
844. 比较含退格的字符串
简单
相关标签
premium lock icon
相关企业
给定 s 和 t 两个字符串，当它们分别被输入到空白的文本编辑器后，如果两者相等，返回 true 。# 代表退格字符。

注意：如果对空文本输入退格字符，文本继续为空。



示例 1：

输入：s = "ab#c", t = "ad#c"
输出：true
解释：s 和 t 都会变成 "ac"。
示例 2：

输入：s = "ab##", t = "c#d#"
输出：true
解释：s 和 t 都会变成 ""。
示例 3：

输入：s = "a#c", t = "b"
输出：false
解释：s 会变成 "c"，但 t 仍然是 "b"。


提示：

1 <= s.length, t.length <= 200
s 和 t 只含有小写字母以及字符 '#'


进阶：

你可以用 O(n) 的时间复杂度和 O(1) 的空间复杂度解决该问题吗？
 */
struct Solution;

impl Solution {
    // 解法一：栈模拟法 - 直观易懂
    // 时间复杂度：O(n + m)，空间复杂度：O(n + m)
    pub fn backspace_compare(s: String, t: String) -> bool {
        // 辅助函数：处理字符串，返回最终的有效字符
        fn process_string(s: String) -> Vec<char> {
            let mut stack = Vec::new();
            
            for ch in s.chars() {
                if ch == '#' {
                    // 遇到退格符，删除栈顶元素（如果栈不为空）
                    stack.pop();
                } else {
                    // 遇到普通字符，入栈
                    stack.push(ch);
                }
            }
            
            stack
        }
        
        // 分别处理两个字符串，然后比较结果
        let processed_s = process_string(s);
        let processed_t = process_string(t);
        
        processed_s == processed_t
    }
    
    // 解法二：双指针法 - 进阶解法（空间复杂度O(1)）
    // 时间复杂度：O(n + m)，空间复杂度：O(1)
    pub fn backspace_compare_advanced(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        
        // 从右往左遍历，因为退格是影响前面的字符
        let mut i = s_chars.len() as i32 - 1;  // s的指针
        let mut j = t_chars.len() as i32 - 1;  // t的指针
        let mut skip_s = 0;  // s需要跳过的字符数
        let mut skip_t = 0;  // t需要跳过的字符数
        
        // 当两个指针都还没遍历完时
        while i >= 0 || j >= 0 {
            // 处理字符串s
            while i >= 0 {
                if s_chars[i as usize] == '#' {
                    // 遇到退格符，增加跳过计数
                    skip_s += 1;
                    i -= 1;
                } else if skip_s > 0 {
                    // 需要跳过这个字符（被前面的#删除了）
                    skip_s -= 1;
                    i -= 1;
                } else {
                    // 找到了一个有效字符
                    break;
                }
            }
            
            // 处理字符串t（逻辑同上）
            while j >= 0 {
                if t_chars[j as usize] == '#' {
                    skip_t += 1;
                    j -= 1;
                } else if skip_t > 0 {
                    skip_t -= 1;
                    j -= 1;
                } else {
                    break;
                }
            }
            
            // 比较当前找到的有效字符
            if i >= 0 && j >= 0 {
                // 两个字符串都还有有效字符
                if s_chars[i as usize] != t_chars[j as usize] {
                    return false;  // 字符不匹配
                }
            } else if i >= 0 || j >= 0 {
                // 一个字符串还有字符，另一个没有了
                return false;
            }
            
            // 移动到下一个字符
            i -= 1;
            j -= 1;
        }
        
        true  // 所有字符都匹配
    }

    pub fn backspace_compare2(s: String, t: String) -> bool {
        let s_chars: Vec<_> = s.chars().collect();
        let t_chars: Vec<_> = t.chars().collect();
        let mut s_inx:i32 = s.len() as i32 - 1;
        let mut t_inx:i32 = t.len() as i32 - 1;

        while s_inx >= 0 || t_inx >= 0 {
            let mut s_skip = 0;
            let mut t_skip = 0;
            while s_inx >= 0{
                if s_chars[s_inx as usize] == '#' {
                    s_inx -= 1;
                    s_skip += 1;
                }else if s_skip > 0{
                    s_inx -= 1;
                    s_skip -= 1;
                }else{
                    break;
                }
            }

            while t_inx >= 0 {
                if t_chars[t_inx as usize] == '#' {
                    t_inx -= 1;
                    t_skip += 1;
                }else if t_skip > 0 {
                    t_inx -= 1;
                    t_skip -= 1;
                }else {
                    break;
                }
            }
            if s_inx >= 0 && t_inx >= 0 {
                if s_chars[s_inx as usize] != t_chars[t_inx as usize] {
                    return false;
                }
            }else if s_inx>=0 || t_inx >= 0{
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("=== LeetCode 844: 比较含退格的字符串 ===\n");
    
    // 测试用例1
    let s1 = String::from("ab#c");
    let t1 = String::from("ad#c");
    println!("测试1:");
    println!("s = \"{}\", t = \"{}\"", s1, t1);
    println!("s处理后: \"ab#c\" -> \"ac\"");
    println!("t处理后: \"ad#c\" -> \"ac\"");
    let result1 = Solution::backspace_compare2(s1.clone(), t1.clone());
    let result1_adv = Solution::backspace_compare_advanced(s1, t1);
    println!("栈模拟法结果: {}", result1);
    println!("双指针法结果: {}", result1_adv);
    println!();
    
    // 测试用例2
    let s2 = String::from("ab##");
    let t2 = String::from("c#d#");
    println!("测试2:");
    println!("s = \"{}\", t = \"{}\"", s2, t2);
    println!("s处理后: \"ab##\" -> \"\"");
    println!("t处理后: \"c#d#\" -> \"\"");
    let result2 = Solution::backspace_compare(s2.clone(), t2.clone());
    let result2_adv = Solution::backspace_compare_advanced(s2, t2);
    println!("栈模拟法结果: {}", result2);
    println!("双指针法结果: {}", result2_adv);
    println!();
    
    // 测试用例3
    let s3 = String::from("a#c");
    let t3 = String::from("b");
    println!("测试3:");
    println!("s = \"{}\", t = \"{}\"", s3, t3);
    println!("s处理后: \"a#c\" -> \"c\"");
    println!("t处理后: \"b\" -> \"b\"");
    let result3 = Solution::backspace_compare(s3.clone(), t3.clone());
    let result3_adv = Solution::backspace_compare_advanced(s3, t3);
    println!("栈模拟法结果: {}", result3);
    println!("双指针法结果: {}", result3_adv);
    println!();
    
    // 复杂测试用例
    let s4 = String::from("a##c#d#");
    let t4 = String::from("#a#c");
    println!("测试4:");
    println!("s = \"{}\", t = \"{}\"", s4, t4);
    println!("s处理后: \"a##c#d#\" -> \"\" (a被删除，再删除一个空字符，c被删除，d被删除)");
    println!("t处理后: \"#a#c\" -> \"c\" (删除空字符，a被删除，剩下c)");
    let result4 = Solution::backspace_compare(s4.clone(), t4.clone());
    let result4_adv = Solution::backspace_compare_advanced(s4, t4);
    println!("栈模拟法结果: {}", result4);
    println!("双指针法结果: {}", result4_adv);
}

/*
算法详解：

=== 解法一：栈模拟法 ===
思路：
1. 用栈来模拟文本编辑器的行为
2. 遇到普通字符就入栈
3. 遇到 '#' 就出栈（相当于删除前一个字符）
4. 最后比较两个栈的内容是否相同

步骤演示（以 "ab#c" 为例）：
- 读取 'a' → stack: ['a']
- 读取 'b' → stack: ['a', 'b']  
- 读取 '#' → stack: ['a'] (删除 'b')
- 读取 'c' → stack: ['a', 'c']
- 最终结果: "ac"

=== 解法二：双指针法 ===
思路：
1. 从右往左遍历两个字符串（因为退格影响的是前面的字符）
2. 用两个计数器记录需要跳过的字符数
3. 遇到 '#' 就增加跳过计数
4. 遇到普通字符且有跳过计数时，减少计数并跳过该字符
5. 找到有效字符时进行比较

为什么从右往左？
- 因为 '#' 影响的是它前面的字符
- 从右往左可以先处理 '#'，再决定前面的字符是否有效

步骤演示（以 "ab#c" 为例）：
- 从右往左: c, #, b, a
- 读取 'c' → 有效字符，记录
- 读取 '#' → skip_count = 1
- 读取 'b' → 被跳过（skip_count = 0）
- 读取 'a' → 有效字符，记录
- 最终有效字符: ['a', 'c']

复杂度分析：
- 栈模拟法：时间 O(n+m)，空间 O(n+m)
- 双指针法：时间 O(n+m)，空间 O(1)
*/