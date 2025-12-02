package hot100_java.lc49;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * 49. 字母异位词分组
 * 排序后作为键，HashMap 聚合。
 */
public class LC049GroupAnagrams {

    public static List<List<String>> groupAnagrams(String[] strs) {
        Map<String, List<String>> groups = new HashMap<>();
        for (String s : strs) {
            char[] arr = s.toCharArray();
            Arrays.sort(arr);
            String key = new String(arr);
            groups.computeIfAbsent(key, k -> new ArrayList<>()).add(s);
        }
        return new ArrayList<>(groups.values());
    }

    public static void main(String[] args) {
        String[][] samples = {
                {"eat", "tea", "tan", "ate", "nat", "bat"},
                {""},
                {"a"}
        };
        for (String[] sample : samples) {
            System.out.println(groupAnagrams(sample));
        }
    }
}
