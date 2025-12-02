package hot100_java.lc438;

import java.util.*;

public class LC438 {
    class Solution {
        public List<Integer> findAnagrams(String s, String p) {
            // 存储结果
            List<Integer> result = new ArrayList<>();

            // 如果 s 长度小于 p，则不可能存在匹配项
            if (s.length() < p.length()) {
                return result;
            }

            // 记录 p 中各个字符出现的次数
            Map<Character, Integer> need = new HashMap<>();
            for (char c : p.toCharArray()) {
                need.put(c, need.getOrDefault(c, 0) + 1);
            }

            // 滑动窗口相关变量
            int left = 0, right = 0;
            int valid = 0; // 表示窗口中满足 need 条件的字符个数

            // 记录窗口中的字符及其数量
            Map<Character, Integer> window = new HashMap<>();

            while (right < s.length()) {
                char rChar = s.charAt(right);
                right++;

                // 更新窗口内的数据
                if (need.containsKey(rChar)) {
                    window.put(rChar, window.getOrDefault(rChar, 0) + 1);
                    if (window.get(rChar).equals(need.get(rChar))) {
                        valid++;
                    }
                }

                // 判断左侧窗口是否要收缩
                while (right - left >= p.length()) {
                    // 当窗口符合条件时，把起始索引加入 result
                    if (valid == need.size()) {
                        result.add(left);
                    }

                    char lChar = s.charAt(left);
                    left++;

                    // 更新窗口内的数据
                    if (need.containsKey(lChar)) {
                        if (window.get(lChar).equals(need.get(lChar))) {
                            valid--;
                        }
                        window.put(lChar, window.get(lChar) - 1);
                    }
                }
            }

            return result;
        }
    }

    public static void main(String[] args) {
        System.out.println(new LC438().new Solution().findAnagrams("cbaebabacd", "abc"));
    }
}
