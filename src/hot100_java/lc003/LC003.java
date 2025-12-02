package hot100_java.lc003;

import java.util.HashSet;

/**
 * 3. 无重复字符的最长子串
 * 中等
 * 相关标签
 * premium lock icon相关企业
 * 提示
 * <p>
 * 给定一个字符串 s ，请你找出其中不含有重复字符的 最长
 * <p>
 * 的长度。
 * <p>
 * <p>
 * <p>
 * 示例 1:
 * <p>
 * 输入: s = "abcabcbb"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。注意 "bca" 和 "cab" 也是正确答案。
 * <p>
 * 示例 2:
 * <p>
 * 输入: s = "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 * <p>
 * 示例 3:
 * <p>
 * 输入: s = "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 * <p>
 * <p>
 * <p>
 * 提示：
 * <p>
 * 0 <= s.length <= 5 * 104
 * s 由英文字母、数字、符号和空格组成
 */

public class LC003 {

    class Solution {
        public int lengthOfLongestSubstring(String s) {
            if (s == null || s.isEmpty()) return 0;
            if (s.length() == 1) return 1;
            char[] chars = s.toCharArray();
            int len = chars.length;
            int result = 0;

            for (int L = 0; L < len; L++) {
                HashSet<Character> set = new HashSet<>();
                // 暴力加减枝
                if (result > len - L) {
                    break;
                }
                for (int R = L; R < len; R++) {
                    if (set.contains(chars[R])) {
                        break;
                    } else {
                        set.add(chars[R]);
                    }
                }
                result = Math.max(result, set.size());
            }

            return result;
        }
    }

    public static void main(String[] args) {
        System.out.println(new LC003().new Solution().lengthOfLongestSubstring("abcabcbb"));
    }

}
