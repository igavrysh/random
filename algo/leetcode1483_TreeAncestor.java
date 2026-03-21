public class leetcode1483_TreeAncestor {
    private int LOG = 0;
    private int[][] up;
    public leetcode1483_TreeAncestor(int n, int[] parent) {
       int val = 1;
        while (val < n) {
            val = val << 1;
            LOG++;
        }
        up = new int[n][LOG];
        parent[0] = -1;

        for (int i=0;i<n;i++) {
            up[i][0] = parent[i];
        }
        
        for (int j=1;j<LOG;j++) {
            for (int i=0;i<n;i++) {
                up[i][j] = up[i][j-1] != -1 ? up[up[i][j-1]][j-1] : -1;
            }
        }
    }
    public int getKthAncestor(int node, int k) {
        for (int i=0;i<LOG;i++) {
            if ((k & (1 << i)) > 0) {
                node = up[node][i];
                if (node == -1) {
                    break;
                }
            }
        }
        return node;
    }
    public static void main(String[] args) {
        test2();
        test1();
    }
    public static void test2() {
        leetcode1483_TreeAncestor sol = new leetcode1483_TreeAncestor(
            2, new int[]{-1,0});
        int output = sol.getKthAncestor(1, 2);
        boolean passed = output == -1;
        System.out.println("test2:" + (passed ? "passed" : "failed"));
    }
    public static void test1() {
        leetcode1483_TreeAncestor sol = new leetcode1483_TreeAncestor(
            4, new int[]{-1,2,3,0});
        int output = sol.getKthAncestor(2, 2);
        boolean passed = output == 0;
        System.out.println("test1:" + (passed ? "passed" : "failed"));
    }
}
