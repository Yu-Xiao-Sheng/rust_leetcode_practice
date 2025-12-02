import java.util.Arrays;

/**
 * 283. 移动零
 * 快慢指针覆盖，最后填零。
 */
public class LC283MoveZeroes {

    public static void moveZeroes(int[] nums) {
        int slow = 0;
        for (int fast = 0; fast < nums.length; fast++) {
            if (nums[fast] != 0) {
                nums[slow] = nums[fast];
                slow++;
            }
        }
        while (slow < nums.length) {
            nums[slow++] = 0;
        }
    }

    public static void main(String[] args) {
        int[] a = {0, 1, 0, 3, 12};
        moveZeroes(a);
        System.out.println(Arrays.toString(a)); // [1,3,12,0,0]

        int[] b = {0};
        moveZeroes(b);
        System.out.println(Arrays.toString(b)); // [0]
    }
}
