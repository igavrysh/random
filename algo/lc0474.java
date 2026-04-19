public class lc0474 {
    private int dp(int i, int[][] strs, int zeros, int ones, Integer[][][] dp) {
        if (i==strs.length) {
            if (zeros>=0 && ones>=0) {
                return 0;
            } else {
                return -1;
            }
        }
        if (zeros<0 || ones<0) {
            return -1;
        }
        if (dp[i][zeros][ones] != null) {
            return dp[i][zeros][ones];
        }
        int max_length = 0;
        int max_length_taking = dp(i+1, strs, zeros-strs[i][0], ones-strs[i][1], dp);
        if (max_length_taking != -1) {
            max_length = Math.max(max_length, 1+max_length_taking);
        }
        int max_length_skipping = dp(i+1, strs, zeros, ones, dp);
        max_length = Math.max(max_length, max_length_skipping);
        dp[i][zeros][ones] = max_length;
        return max_length;
    }

    public int findMaxForm(String[] strs, int m, int n) {
        int[][] stats = new int[strs.length][2];
        for (int i=0;i<strs.length;i++) {
            for (char c : strs[i].toCharArray()) {
                if (c == '0') {
                    stats[i][0]++;
                } else {
                    stats[i][1]++;
                }
            }
        }
        Integer[][][] dp = new Integer[strs.length][m+1][n+1];
        dp(0, stats, m, n, dp);
        int max_len = 0;
        for (int i=0;i<=m;i++) {
            for (int j=0;j<=n;j++) {
                if (dp[0][i][j] == null) {
                    continue;
                }
                max_len = Math.max(max_len, dp[0][i][j]);
            } 
        }
        return max_len;
    }

    public static void test1() {
        String[] strs = new String[] {"00011","00001","00001","0011","111"};
        lc0474 sol = new lc0474();
        int output = sol.findMaxForm(strs, 8, 5);
        boolean passed = output == 3;
        System.out.println("test1:" + (passed ? "passed" : "failed"));
    }

    public static void main(String[] args) {
        test1();
    }
}
