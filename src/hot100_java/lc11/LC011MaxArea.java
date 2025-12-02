package hot100_java.lc11;

/**
 * 11. 盛最多水的容器
 * 双指针：收缩矮板，时间 O(n)，空间 O(1)。
 */
public class LC011MaxArea {

    public static int maxArea(int[] height) {
        int i = 0, j = height.length - 1;
        int ans = 0;
        while (i < j) {
            int h = Math.min(height[i], height[j]);
            ans = Math.max(ans, h * (j - i));
            if (height[i] < height[j]) {
                i++;
            } else {
                j--;
            }
        }
        return ans;
    }

    public static void main(String[] args) {
        int res = maxArea(new int[]{1, 8, 6, 2, 5, 4, 8, 3, 7});
        System.out.println(res); // 49
    }
}
