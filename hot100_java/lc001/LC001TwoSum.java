import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

/**
 * 1. 两数之和
 * JDK8 单文件可直接运行：javac LC001TwoSum.java && java LC001TwoSum
 */
public class LC001TwoSum {

    public static int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> seen = new HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            int need = target - nums[i];
            if (seen.containsKey(need)) {
                return new int[]{i, seen.get(need)};
            }
            seen.put(nums[i], i);
        }
        return new int[0];
    }

    public static void main(String[] args) {
        int[] nums = {2, 11, 7, 15};
        int[] res = twoSum(nums, 9);
        System.out.println(Arrays.toString(res));
    }
}
