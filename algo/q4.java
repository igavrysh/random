import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

class q4 {
    public boolean canPartitionGrid(int[][] grid) {
        int r = grid.length;
        int c = grid[0].length;
        long[] rows = new long[r];
        long[] cols = new long[c];
        HashMap<Integer, ArrayList<int[]>> m = new HashMap<>();
        for (int i=0;i<r;i++) {
            for (int j=0;j<c;j++) {
                rows[i] += grid[i][j];
                cols[j] += grid[i][j];
                ArrayList<int[]> a = m.getOrDefault(grid[i][j], new ArrayList<>());
                a.add(new int[]{i,j});
                m.put(grid[i][j], a);
            }
        }
        long[] p_rows = new long[r];
        for (int i=0;i<r;i++) {
            p_rows[i] = rows[i] + (i>0 ? p_rows[i-1] : 0);
        }
        long[] p_cols = new long[c];
        for (int j=0;j<c;j++) {
            p_cols[j] = cols[j] + (j>0 ? p_cols[j-1] : 0);
        }
        
        for (int i=0;i<r-1;i++) {
            long delta = p_rows[i] - (p_rows[r-1]-p_rows[i]);
            if (delta == 0) { return true; }
            if (delta > 0) {
                List<int[]> a = m.get((int)Math.abs(delta));
                if (a == null) {
                    continue;
                }
                for (int k=0;k<a.size();k++) {
                    int[] p = a.get(k);
                    if (p[0]<=i && (i>0 || isEdgeCell(p,r,c))) {
                        return true;
                    }
                }
            } else {
                List<int[]> a = m.get((int)Math.abs(delta));
                if (a==null) {
                    continue;
                }
                for (int k=0;k<a.size();k++) {
                    int[] p = a.get(k);
                    if (p[0]>i && (r-1-i>1 || isEdgeCell(p,r,c))) {
                        return true;
                    }
                }
            }
        }
        for (int j=0;j<c-1;j++) {
            long delta = p_cols[j] - (p_cols[c-1]-p_cols[j]);
            if (delta == 0) {
                return true;
            }
            if (delta > 0) {
                List<int[]> a = m.get((int)Math.abs(delta));
                if (a==null) {
                    continue;
                }
                for (int k=0;k<a.size();k++) {
                    int[] p = a.get(k);
                    if (p[1]<=j && (j>0 || isEdgeCell(p,r,c))) {
                        return true;
                    }
                }
            } else {
                List<int[]> a = m.get((int)Math.abs(delta));
                if (a==null) {
                    continue;
                }
                for (int k=0;k<a.size();k++) {
                    int[] p = a.get(k);
                    if (p[1]>j && (c-1-j>1 || isEdgeCell(p,r,c))) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    private boolean isEdgeCell(int[] pos, int r, int c) {
        return (pos[0]==0 && pos[1]==0)
            || (pos[0]==r-1 && pos[1]==0)
            || (pos[0]==0 && pos[1]==c-1)
            || (pos[0]==r-1 && pos[1]==c-1);
    }

    public static void main(String[] args) {
        test678();
        test1();
    }

    public static void test1() {
        int[][] grid = {{1,2},{3,4}};
        q4 sol = new q4();
        boolean output = sol.canPartitionGrid(grid);
        boolean exp_output = true;
        boolean passed = output == exp_output;
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }

    public static void test678() {
        int[][] grid = {{1,2,4},{2,3,5}};
        q4 sol = new q4();
        boolean output = sol.canPartitionGrid(grid);
        boolean exp_output = false;
        boolean passed = output == exp_output;
        System.out.println("test678: " + (passed ? "passed" : "failed"));
    }
}