import java.util.ArrayList;

class lc2081 {
    public long kMirror(int k, int n) {
        int counter = 0;
        int num=1;
        long sum=0;
        while (counter<n) {
            if (is_k_mirror(num, k)) {
                sum+=num;
                counter++;
            }
            num++;
        }
        return sum;
    }
    private boolean is_k_mirror(int num, int k) {
        ArrayList<Integer> base10 = new ArrayList<>();
        ArrayList<Integer> basek = new ArrayList<>();
        int num_10 = num;
        int num_k = num;
        while (num_10 != 0 || num_k != 0) {
            if (num_10 != 0) {
                base10.add(num_10%10);
            }
            if (num_k != 0) {
                basek.add(num_k%k);
            }
            num_10/=10;
            num_k/=k;
        }
        for (int i=0;i<base10.size()/2;i++) {
            if (base10.get(i)!=base10.get(base10.size()-1-i)) {
                return false;
            }
        }
        for (int i=0;i<basek.size()/2;i++) {
            if (basek.get(i)!=basek.get(basek.size()-1-i)) {
                return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        test1();
    }

    public static void test1() {
        int k = 2;
        int n = 5;
        lc2081 sol = new lc2081();
        long res = sol.kMirror(k, n);
        boolean passed = res == 25;
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }
}