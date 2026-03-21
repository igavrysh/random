public class leetcode2040 {
    public long kthSmallestProduct(int[] nums1, int[] nums2, long k) {
        int nums2_m=nums2.length;
        for (int i=0;i<nums2.length;i++) {
            if (nums2[i]>=0) {
                nums2_m=i;
                break;
            }
        }
        long bad = -10000000001l;
        long good = 10000000001l;
        while (good-bad>1) {
            long m = bad + (good-bad)/2;
            long acc = 0;
            for (int i=0;i<nums1.length;i++) {
                if (nums1[i]<0) {
                    if (nums2_m>0) {
                        acc += bs(0,nums2_m-1,nums2,nums1[i],m,false);
                    }
                    if (nums2_m<nums2.length) {
                        acc += bs(nums2_m,nums2.length-1,nums2,nums1[i],m,false);
                    }
                } else {
                    acc += bs(0,nums2.length-1,nums2,nums1[i],m,true);
                }
            }
            if (acc < k) {
                bad = m;
            } else {
                good = m;
            }
        }
        return good;
    }
    public static int bs(int l, int r, int[] nums, int mult, long th, boolean is_lower) {
        int bad = l-1;
        int good = r+1;
        while (good-bad>1) {
            int m=bad+(good-bad)/2;
            if (is_lower) {
                if (nums[m]*(long)mult>th) {
                    good = m;
                } else {
                    bad = m;
                }
            } else {
                if (nums[m]*(long)mult>th) {
                    bad = m;
                } else {
                    good = m;
                }
            }
        }
        if (is_lower) {
            return bad-l+1;
        }
        return r-good+1;
    }

    public static void main(String[] args) {
        test2();
        test1();
    }

    public static void test1() {
        int[] nums1 = {2,5};
        int[] nums2 = {3,4};
        int k = 2;
        leetcode2040 sol = new leetcode2040();
        long output = sol.kthSmallestProduct(nums1, nums2, k);
        long exp_output = 8;
        boolean passed = output == exp_output;
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }

    public static void test2() {
        int[] nums1 = {-4,-2,0,3};
        int[] nums2 = {2,4};
        int k = 6;
        leetcode2040 sol = new leetcode2040();
        long output = sol.kthSmallestProduct(nums1, nums2, k);
        long exp_output = 0;
        boolean passed = output == exp_output;
        System.out.println("test2: " + (passed ? "passed" : "failed"));
    }
}
