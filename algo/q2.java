import java.util.Arrays;
import java.util.HashMap;
class q2_Solution {
    public int minSwaps(int[] nums) {
        int n = nums.length;
        Integer[] a = new Integer[n];
        for (int i=0;i<n;i++) {
            a[i] = nums[i];
        }
        Arrays.sort(a, (Integer i1, Integer i2) -> {
            int i1_s = sum_digits(i1);
            int i2_s = sum_digits(i2);
            if (i1_s == i2_s) {
                if (i1 > i2) {
                    return 1;
                } else {
                    return -1;
                }
            } else {
                if (i1_s > i2_s) {
                    return 1;
                } else {
                    return -1;
                }
            }
        });
        HashMap<Integer, Integer> m_a = new HashMap<>();
        HashMap<Integer, Integer> m_nums = new HashMap<>();
        for (int i=0;i<n;i++) {
            m_a.put(a[i], i);
            m_nums.put(nums[i], i);
        }
        int counter = 0;
        for (int i=0;i<n;i++) {
            int tmp = nums[i];
            while (m_a.get(tmp) != m_nums.get(tmp)) {
                int tmp_i = m_nums.get(tmp);
                int tmp_j = m_a.get(tmp);
                int next_tmp = nums[tmp_j];
                nums[tmp_i] = tmp;
                nums[tmp_j] = next_tmp;
                tmp = next_tmp;
                counter++;
            }
        }
        return counter;
    }
    private int sum_digits(int num) {
        int sum = 0;
        while (num>0) {
            sum+=num%10;
            num/=10;
        }
        return sum;
    }
    public static void main(String[] args) {
        test1();
    }
    public static void test1() {
        int[] nums = {37,100};
        q2_Solution sol = new q2_Solution();
        int t = sol.minSwaps(nums);
    }
}