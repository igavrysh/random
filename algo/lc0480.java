
import java.util.Map.Entry;
import java.util.TreeMap;

public class lc0480 {
    private void rebalance(TreeMap<Integer, Integer> top, TreeMap<Integer, Integer> bottom, int k, int[] sizes) {
        if (top.size() > 0 && bottom.size()>0) {
            if (top.firstKey() < bottom.lastKey()) {
                Entry<Integer, Integer> bottomLast = bottom.pollLastEntry();
                sizes[0] -= bottomLast.getValue();
                Entry<Integer, Integer> topFirst = top.pollFirstEntry();
                sizes[1] -= topFirst.getValue();

                if (topFirst.getValue() > 1) {
                    top.put(topFirst.getKey(), topFirst.getValue()-1);
                    sizes[1] += topFirst.getValue()-1;
                }

                if (bottomLast.getValue() > 1) {
                    bottom.put(bottomLast.getKey(), bottomLast.getValue()-1);
                    sizes[0] = bottomLast.getValue()-1;
                }
                //sizes[1] = 1;
                bottom.put(topFirst.getKey(), bottom.getOrDefault(topFirst.getKey(), 0)+1);
                sizes[0] += 1;
                top.put(bottomLast.getKey(), top.getOrDefault(bottomLast.getKey(), 0)+1);
                sizes[1] += 1;


                //if (topFirst.getValue() != 1 || bottomLast.getValue() != 1) {
                //    throw new RuntimeException("unexpected state reached");
                //}
                // probably not required...
                //if (topFirst.getKey() > 1) {
                //    top.put(topFirst.getKey(), topFirst.getValue()-1);
                //}
                // probably not required...
                //if (bottomLast.getKey() > 1) {
                //    bottom.put(bottomLast.getKey(), bottomLast.getValue()-1);
                //}
                //bottom.put(topFirst.getKey(),1);
                //top.put(bottomLast.getKey(), 1);
            }
        }
    }

    public double[] medianSlidingWindow(int[] nums, int k) {
        TreeMap<Integer, Integer> top = new TreeMap<>();
        TreeMap<Integer, Integer> bottom = new TreeMap<>();
        int n = nums.length;
        double[] res = new double[n-k+1];
        int[] sizes = {0,0};
        for (int i=0;i<n;i++) {
             if (i-k>=0) {
                if (top.containsKey(nums[i-k])) {
                    int fq = top.remove(nums[i-k]);
                    if (fq > 1) {
                        top.put(nums[i-k], fq-1);
                    }
                    sizes[1]--;
                } else {
                    int fq = bottom.remove(nums[i-k]);
                    if (fq > 1) {
                        bottom.put(nums[i-k], fq-1);
                    }
                    sizes[0]--;
                }
            }
            bottom.put(nums[i], bottom.getOrDefault(nums[i], 0)+1);
            sizes[0]++;
            if (sizes[0]>k/2) {
                int last = bottom.lastKey();
                int fq = bottom.remove(last);
                sizes[0]--;
                if (fq > 1) {
                    bottom.put(last, fq-1);
                }
                top.put(last, top.getOrDefault(last, 0)+1);
                sizes[1]++;
            }
            rebalance(top, bottom, k, sizes);
            if (i-k+1<0) {
                continue;
            }
            if (k%2==0) {
                res[i-k+1] = (bottom.lastKey() + top.firstKey()*1.0)/2.0;
            } else {
                res[i-k+1] = top.firstKey();
            }
        }
        return res;
    }

    public static void test2() {
        lc0480 sol = new lc0480();
        int[] nums = {2147483647,2147483647};
        int k = 2;
        double[] expectedOutput = {2147483647};
        double[] output = sol.medianSlidingWindow(nums, k);
        boolean passed = expectedOutput.length == output.length;
        if (passed) {
            for (int i=0;i<expectedOutput.length;i++) {
                if (Math.abs(output[i]-expectedOutput[i])>0.0001) {
                    passed = false;
                    break;
                }
            }
        }
        System.out.println("test2: " + (passed ? "passed" : "failed"));
    }

    public static void test1() {
        lc0480 sol = new lc0480();
        int[] nums = {1,3,-1,-3,5,3,6,7};
        int k = 3;
        double[] expectedOutput = {1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000};
        double[] output = sol.medianSlidingWindow(nums, k);
        boolean passed = expectedOutput.length == output.length;
        if (passed) {
            for (int i=0;i<expectedOutput.length;i++) {
                if (Math.abs(output[i]-expectedOutput[i])>0.0001) {
                    passed = false;
                    break;
                }
            }
        }
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }

    public static void main(String[] args) {
        test1();
        test2();
    }
}