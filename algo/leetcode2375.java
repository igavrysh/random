class leetcode2375 {
    public String smallestNumber(String pattern) {
        boolean[] used = new boolean[10];
        StringBuilder sb = new StringBuilder();
        for (int i=1;i<used.length;i++) {
            used[i] = true;
            sb.append(""+i);
            String res = bt(0,sb,pattern,used);
            if (res != null) {
                return res;
            }
            used[i] = false;
            sb.deleteCharAt(sb.length()-1);
        }
        return null;
    }
    public String bt(int pos, StringBuilder sb, String pattern, boolean[] used) {
        if (pos==pattern.length()) {
            return sb.toString();
        }
        char ch = pattern.charAt(pos);
        int prev_num = (int)(sb.charAt(sb.length()-1)-'0');
        if (ch == 'I') {
            for (int i=prev_num+1;i<used.length;i++) {
                if (used[i]) { continue; }
                used[i] = true;
                sb.append(""+i);
                //System.out.println("curr sb:" + sb.toString());
                String res = bt(pos+1, sb, pattern, used);
                if (res != null) {
                    return res;
                }
                used[i] = false;
                sb.deleteCharAt(sb.length()-1);
            }
        }
        if (ch == 'D') {
            for (int i=1;i<prev_num;i++) {
                if (used[i]) { continue; }
                used[i] = true;
                sb.append(""+i);
                //System.out.println("curr sb:" + sb.toString());
                String res = bt(pos+1, sb, pattern, used);
                if (res != null) {
                    return res;
                }
                used[i] = false;
                sb.deleteCharAt(sb.length()-1);
            }
        }
        return null;
    }
    public static void main(String[] args) {
        test2();
        test1();
    }
    public static void test2() {
        String s = "DDD";
        String exp_output = "4321";
        leetcode2375 sol = new leetcode2375();
        String output = sol.smallestNumber(s);
        boolean passed = (exp_output == null && output == null) || (output != null && output.equals(exp_output));
        System.out.println("test2: " + (passed ? "passed" : "failed"));
    }
    public static void test1() {
        String s = "IIIDIDDD";
        String exp_output = "123549876";
        leetcode2375 sol = new leetcode2375();
        String output = sol.smallestNumber(s);
        boolean passed = (exp_output == null && output == null) || (output != null && output.equals(exp_output));
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }
}