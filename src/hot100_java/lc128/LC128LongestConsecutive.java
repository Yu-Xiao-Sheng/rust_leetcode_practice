package hot100_java.lc128;

import java.util.HashSet;
import java.util.Set;

/**
 * 128. 最长连续序列
 * 哈希表判前驱，时间 O(n)，空间 O(n)。
 */
public class LC128LongestConsecutive {

    public static int longestConsecutive(int[] nums) {
        Set<Integer> set = new HashSet<>();
        for (int v : nums) {
            set.add(v);
        }
        if (set.isEmpty()) {
            return 0;
        }

        int best = 0;
        int half = (set.size() + 1) / 2; // 早停阈值
        for (int num : set) {
            if (set.contains(num - 1)) {
                continue; // 不是序列起点
            }
            int cur = num;
            int len = 1;
            while (set.contains(cur + 1)) {
                cur++;
                len++;
            }
            best = Math.max(best, len);
            if (best >= half) {
                return best;
            }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(longestConsecutive(new int[]{100, 4, 200, 1, 3, 2})); // 4
        System.out.println(longestConsecutive(new int[]{0, 3, 7, 2, 5, 8, 4, 6, 0, 1})); // 9
        System.out.println(longestConsecutive(new int[]{1, 0, 1, 2})); // 3
    }
}
