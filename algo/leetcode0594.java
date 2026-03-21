import java.util.Arrays;
class leetcode0594 {
    public int findLHS(int[] nums) {
        Arrays.sort(nums);
        int min=nums[0]-2;
        int min_count = 0;
        int max=nums[0];
        int max_count = 1;
        int max_len = 0;
        for (int i=1;i<nums.length;i++) {
            if (nums[i] == min) {
                min_count++;
            } else if (nums[i] == max) {
                max_count++;
            } else {
                if (max-min==1) {
                    max_len = Math.max(max_len, min_count+max_count);
                }
                if (nums[i]==max+1) {
                    min = max;
                    min_count = max_count;
                    max = nums[i];
                    max_count = 1;
                } else {
                    min=nums[i]-2;
                    min_count = 0;
                    max=nums[i];
                    max_count = 1;
                }
            }
        }
        if (max-min==1) {
            max_len = Math.max(max_len, max_count+min_count);
        }
        return max_len;
    }

    public static void main(String[] args) {
        test4();
        test3();
        test2();
        test1();
    }

    public static void test4() {
        int[] nums = {1,2,3,3,1,-14,13,4};
        leetcode0594 sol = new leetcode0594();
        int exp_output = 3;
        int output = sol.findLHS(nums);
        boolean passed = exp_output == output;
        System.out.println("test4: " + (passed ? "passed" : "failed"));
    }

    public static void test3() {
        int[] nums = {-1,0,-1,0,-1,0,-1};
        leetcode0594 sol = new leetcode0594();
        int exp_output = 7;
        int output = sol.findLHS(nums);
        boolean passed = exp_output == output;
        System.out.println("test3: " + (passed ? "passed" : "failed"));
    }

    public static void test2() {
        int[] nums = {1,2,2,1};
        leetcode0594 sol = new leetcode0594();
        int exp_output = 4;
        int output = sol.findLHS(nums);
        boolean passed = exp_output == output;
        System.out.println("test2: " + (passed ? "passed" : "failed"));
    }

    public static void test1() {
        int[] nums = {1,3,2,2,5,2,3,7};
        leetcode0594 sol = new leetcode0594();
        int exp_output = 5;
        int output = sol.findLHS(nums);
        boolean passed = exp_output == output;
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }
}