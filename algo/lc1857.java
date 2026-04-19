import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;

class lc1857 {
    public int largestPathValue(String colors, int[][] edges) {
        HashMap<Integer, ArrayList<Integer>> G = new HashMap<>();
        int n = colors.length();
        for (int[] e : edges) {
            if (G.get(e[0]) == null) {
                G.put(e[0], new ArrayList<>());
            }
            G.get(e[0]).add(e[1]);
        }
        int[][] dp = new int[n][26];
        boolean[] visited = new boolean[n];
        for (int v=0;v<n;v++) {
            if (!visited[v]) {
                visited[v] = true;
                HashSet<Integer> path = new HashSet<>();
                path.add(v);
                int[] dp_res = new int[26];
                int v_color = (int)(colors.charAt(v)-'a');
                dp_res[v_color]++;
                int res = dfs(v, G, dp, visited, colors, path);
                if (res == -1) {
                    return -1;
                }
            }
        }
        int max_fq = 0;
        for (int v=0;v<n;v++) {
            for (int i=0;i<26;i++) {
                if (dp[v][i]>max_fq) {
                    max_fq = dp[v][i];
                }
            }
        }
        return max_fq;
    }
    private int dfs(int v, HashMap<Integer, ArrayList<Integer>> G,
        int[][] dp, boolean[] visited, String colors, HashSet<Integer> path
    ) {
        int[] dp_res = new int[26];
        int v_color = (int)(colors.charAt(v)-'a');
        dp_res[v_color]++;
        ArrayList<Integer> next_vs = G.get(v);
        if (next_vs != null) {
            for (int next_v : next_vs) {
                if (path.contains(next_v)) {
                    return -1;
                }
                visited[next_v] = true;
                path.add(next_v);
                int res = dfs(next_v, G, dp, visited, colors, path);
                if (res == -1) {
                    return -1;
                }
                path.remove(next_v);
                for (int i=0;i<26;i++) {
                    dp_res[i] = Math.max(
                        dp_res[i], 
                        dp[next_v][i] + (i==v_color ? 1 : 0)
                    );
                }
            }
        }
        for (int i=0;i<26;i++) {
            dp[v][i] = dp_res[i];
        }
        return 1;
    }

    public static void main(String[] args) {
        test42();
        test21();
    }

    public static void test42() {
        String colors = "qqxfhffrqxqbhhrfrsfxbfxhxxhsfbhbfqqfrsqsqhbrmhmsqxrhfxhffssmrfxhr";
        int[][] edges = {
            {0,1},{1,2},{2,3},{0,3},{3,4},{4,5},{5,6},{6,7},{4,7},{7,8},{6,9},{7,9},{8,9},
            {5,9},{8,10},{7,10},{10,11},{9,11},{8,11},{11,12},{5,12},{11,13},{12,13},{13,14},
            {12,14},{8,14},{10,14},{14,15},{13,15},{12,15},{15,16},{12,16},{8,16},{16,17},{15,18},
            {18,19},{17,19},{19,20},{12,20},{17,20},{20,21},{18,21},{19,22},{21,22},{22,23},{21,23},
            {22,24},{23,25},{24,25},{22,25},{25,26},{26,27},{20,27},{25,28},{13,28},{26,28},{25,29},
            {27,30},{30,31},{13,31},{28,31},{31,32},{26,32},{21,32},{27,32},{30,33},{32,33},{31,33},
            {26,33},{31,34},{25,34},{23,34},{5,35},{32,35},{30,36},{20,36},{29,36},{35,36},{35,37},
            {34,37},{36,37},{32,37},{27,38},{19,39},{28,39},{5,39},{38,40},{39,40},{22,41},{35,41},
            {38,41},{40,41},{24,42},{40,42},{30,43},{40,43},{41,43},{39,44},{22,45},{41,45},{33,45},
            {43,45},{42,46},{43,46},{44,46},{44,47},{30,47},{43,48},{47,48},{48,49},{48,50},{49,50},
            {45,51},{34,51},{37,51},{45,52},{49,53},{36,53},{52,54},{46,54},{53,55},{52,56},{55,56},
            {51,57},{56,57},{50,57},{53,58},{35,58},{43,59},{47,59},{54,59},{45,60},{57,60},{47,60},
            {58,61},{35,61},{61,62},{52,63},{48,63},{47,63},{56,64},{61,64},{52,64}
        };
        lc1857 sol = new lc1857();
        int output = sol.largestPathValue(colors, edges);
        int exp_output = 11;
        boolean passed = output == exp_output;
        System.out.println("test42: " + (passed ? "passed" : "failed"));
    }

    public static void test21() {
        String colors = "hhqhuqhqff";
        int[][] edges = {{0,1},{0,2},{2,3},{3,4},{3,5},{5,6},{2,7},{6,7},{7,8},{3,8},{5,8},{8,9},{3,9},{6,9}};
        lc1857 sol = new lc1857();
        int output = sol.largestPathValue(colors, edges);
        int exp_output = 3;
        boolean passed = output == exp_output;
        System.out.println("test21: " + (passed ? "passed" : "failed"));
    }
}