import java.math.RoundingMode;
import java.text.NumberFormat;
import java.util.ArrayList;
class leetcode2288 {
    public static class Repl {
        public int start_idx;
        public int end_idx;
        public double val;
        public Repl(int start_idx, int end_idx, double val) {
            this.start_idx = start_idx;
            this.end_idx = end_idx;
            this.val = val;
        }
    }
    private Double price(int start_idx, int end_idx, String s) {
        if (start_idx+1==end_idx+1) {
            return null;
        }
        if (s.charAt(start_idx) == '$') {
            String val = s.substring(start_idx+1, end_idx+1).replaceAll("[a-zA-Z]", "y");
            try {
                return Double.valueOf(val);
            } catch (Exception ex) {
                return null;
            }
        }
        return null;
    }
    public String discountPrices(String s, int discount) {
        int n = s.length();
        int prev_space = -1;
        ArrayList<Repl> repls = new ArrayList<>();
        for (int i=0;i<=n;i++) {
            if (i==s.length() || s.charAt(i) == ' ') {
                if (prev_space != i-1) {
                    String substr = s.substring(prev_space+1,i);
                    Double val = price(prev_space+1,i-1,s);
                    if (val != null) {
                        repls.add(new Repl(prev_space+1,i-1,val));
                    }  
                }
                prev_space = i;
            }
        }
        int ptr = 0;
        StringBuilder sb = new StringBuilder();
        NumberFormat nf = NumberFormat.getNumberInstance();
        nf.setMinimumFractionDigits(2);
        nf.setMaximumFractionDigits(2);
        nf.setGroupingUsed(false);
        nf.setRoundingMode(RoundingMode.DOWN);
        for (int i=0;i<n;i++) {
            if (ptr>= repls.size() || i<repls.get(ptr).start_idx) {
                sb.insert(sb.length(), s.charAt(i));
            } else {
                Repl repl = repls.get(ptr);
                sb.insert(sb.length(), '$');
                double val = repl.val*(100.0-discount)/100.0;
                String val_str = nf.format(val);
                sb.insert(sb.length(), val_str);
                i=repl.end_idx;
                ptr++;
            }
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        test3();
        test2();
        test1();
    }

    public static void test3() {
        String sen = "$2f";
        Double d = Double.valueOf("2f"); // equivalent to return new Double(parseDouble(s)); under the hood..., returns 2.0
        Double d2 = Double.parseDouble("2f"); // returns 2.0

        int discount = 50;
        leetcode2288 sol = new leetcode2288();
        String output = sol.discountPrices(sen, discount);
        String exp_output = "$2f";
        boolean passed = output.equals(exp_output);
        System.out.println("test3: " + (passed ? "passed" : "failed"));
    }

    public static void test2() {
        String sen = "ka3caz4837h6ada4 r1 $602";
        int discount = 9;
        leetcode2288 sol = new leetcode2288();
        String output = sol.discountPrices(sen, discount);
        String exp_output = "ka3caz4837h6ada4 r1 $547.82";
        boolean passed = output.equals(exp_output);
        System.out.println("test2: " + (passed ? "passed" : "failed"));
    }

    public static void test1() {
        String sen = "there are $1 $2 and 5$ candies in the shop";
        int discount = 50;
        leetcode2288 sol = new leetcode2288();
        String output = sol.discountPrices(sen, discount);
        String exp_output = "there are $0.50 $1.00 and 5$ candies in the shop";
        boolean passed = output.equals(exp_output);
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }
}