/**
给你一个字符串数组，请你将

组合在一起。可以按任意顺序返回结果列表。



示例 1:

输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]

输出: [["bat"],["nat","tan"],["ate","eat","tea"]]

解释：

在 strs 中没有字符串可以通过重新排列来形成 "bat"。
字符串 "nat" 和 "tan" 是字母异位词，因为它们可以重新排列以形成彼此。
字符串 "ate" ，"eat" 和 "tea" 是字母异位词，因为它们可以重新排列以形成彼此。

示例 2:

输入: strs = [""]

输出: [[""]]

示例 3:

输入: strs = ["a"]

输出: [["a"]]



提示：

1 <= strs.length <= 104
0 <= strs[i].length <= 100
strs[i] 仅包含小写字母

**/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 我的思路，将每一个字符串进行排序，以排序之后的字符串作为键，排序之前的字符串作为值，加入到键对应的列表中；最后整合所有子列表集合就是结果；
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key: String = chars.into_iter().collect();
            groups.entry(key).or_default().push(s);
        }

        groups.into_values().collect()
    }

    pub fn group2(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String,Vec<String>> = HashMap::new();

        for s in strs{
            let mut charts = s.chars().collect::<Vec<char>>();
            charts.sort_unstable();
            let key = String::from_iter(charts);
            groups.entry(key).or_default().push(s);
        }

        groups.into_values().collect()
    }
}

fn sort_for_display(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
    // 排序仅用于打印时保证输出稳定，算法本身不依赖顺序。
    for group in &mut groups {
        group.sort_unstable();
    }
    groups.sort_unstable_by(|a, b| a.cmp(b));
    groups
}

fn main() {
    let samples = vec![
        vec!["eat", "tea", "tan", "ate", "nat", "bat"],
        vec![""],
        vec!["a"],
    ];

    for (i, sample) in samples.iter().enumerate() {
        let input: Vec<String> = sample.iter().map(|s| s.to_string()).collect();
        let grouped = Solution::group2(input);
        let display = sort_for_display(grouped);
        println!("样例 {} 分组结果: {:?}", i + 1, display);
    }
}
