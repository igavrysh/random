import java.util.TreeMap;
import java.util.ArrayList;
import java.util.Iterator;
class leetcode2094 {
    public int[] findEvenNumbers(int[] digits) {
        TreeMap<Integer, Integer> fq = new TreeMap<>((Integer k1, Integer k2) -> Integer.compare(k1,k2));
        for (int i=0;i<digits.length;i++) {
            fq.put(digits[i], fq.getOrDefault(digits[i], 0)+1);
        }
        ArrayList<Integer> res = new ArrayList<>();
        bt(0, fq, res);
        int[] res2 = new int[res.size()];
        for (int i=0;i<res.size();i++) {
            res2[i] = res.get(i);
        }
        return res2;
    }
    private void bt(int num, TreeMap<Integer, Integer> fq, ArrayList<Integer> res) {
        if (num >= 100) {
            if (num%2==0) {
                res.add(num);
            }
            return;
        }
        Iterator<Integer> iterator = fq.keySet().iterator();

        while(iterator.hasNext()) {
            int key = iterator.next();
            int val = fq.get(key);
            if (num==0 && key==0) {
                continue;
            }
            if (val>0) {
                if (val-1>0) {
                    ///fq.put(key, val-1);
                } else {
                    //iterator.remove();
                }
                bt(num*10+key, fq, res);
                //fq.put(key, val);
            }
        }
    }
    public static void main(String[] args) {
        test1();
    }
    public static void test1() {
        int[] nums = {2,1,3,0};
        leetcode2094 sol = new leetcode2094();
        int[] res = sol.findEvenNumbers(nums);
        for (int r : res) {
            System.out.println(r);
        }
    }
}