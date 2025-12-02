package hot100_java.lc42;

/**
 * 42. 接雨水
 * 困难
 * 相关标签
 * premium lock icon相关企业
 * <p>
 * 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
 * <p>
 * <p>
 * <p>
 * 示例 1：
 * <p>
 * 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * 输出：6
 * 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
 * <p>
 * 示例 2：
 * <p>
 * 输入：height = [4,2,0,3,2,5]
 * 输出：9
 * <p>
 * <p>
 * <p>
 * 提示：
 * <p>
 * n == height.length
 * 1 <= n <= 2 * 104
 * 0 <= height[i] <= 105
 */

public class LC042 {
    class Solution {
        public int trap(int[] height) {
            int sum = 0;
            int len = height.length;
            if (len < 3) return 0;

            int[] max_left = new int[len];
            int[] max_right = new int[len];

            for (int i = 1; i < len; i++) {
                max_left[i] = Math.max(max_left[i - 1], height[i]);
            }

            for (int i = len - 2; i >= 0; i--) {
                max_right[i] = Math.max(max_right[i + 1], height[i]);
            }

            for (int i = 1; i < len - 1; i++) {
                int min = Math.min(max_left[i],max_right[i]);
                if (min > height[i]){
                    sum += min - height[i];
                }
            }

            return sum;
        }
    }

    public static void main(String[] args) {
        System.out.println(new LC042().new Solution().trap(new int[]{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}));
    }
}
