package hot100_java.lc15;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * 15. 三数之和
 * 排序 + 双指针去重。
 */
public class LC015ThreeSum {

    public static List<List<Integer>> threeSum(int[] nums) {
        List<List<Integer>> result = new ArrayList<>();
        int len = nums.length;
        if (nums == null || len < 3) return result;

        Arrays.sort(nums);

        for (int i = 0; i < len; i++) {
            if (nums[i] > 0) {
                break;
            }
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }

            int left = i + 1;
            int right = len - 1;

            while (left < right) {
                int sum = nums[i] + nums[left] + nums[right];
                if (sum == 0) {
                    result.add(Arrays.asList(nums[i], nums[left], nums[right]));
                    // 关键去重步骤
                    while (left < right && nums[left + 1] == nums[left]) left++;
                    while (left < right && nums[right - 1] == nums[right]) right--;
                    // 去重之后，真正向内缩短
                    left++;
                    right--;
                } else if (sum > 0) {
                    right--;
                } else if (sum < 0) {
                    left++;
                }
            }
        }

        return result;
    }

    public static void main(String[] args) {
        System.out.println(threeSum(new int[]{-1, 0, 1, 2, -1, -4}));
        System.out.println(threeSum(new int[]{0, 1, 1}));
        System.out.println(threeSum(new int[]{0, 0, 0}));
    }
}
