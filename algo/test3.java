import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.Queue;

public class test3 {
    public int minMoves(String[] matrix) {
        HashMap<Character, ArrayList<int[]>> portals = new HashMap<>();
        int rows = matrix.length;
        int cols = matrix[0].length();
        for (int i=0;i<rows;i++) {
            for (int j=0;j<cols;j++) {
                char ch = matrix[i].charAt(j);
                if (ch-'A'>=0 && ch-'A'<26) {
                    ArrayList<int[]> a = portals.getOrDefault(ch, new ArrayList<int[]>());
                    a.add(new int[]{i,j});
                    portals.put(ch, a);
                }
            }
        }
        Queue<int[]> q = new LinkedList<int[]>();
        boolean[][] visited = new boolean[rows][cols];
        q.offer(new int[]{0,0});
        visited[0][0] = true;
        int[][] dirs = {{-1,0},{1,0},{0,-1},{0,1}};
        int steps = 0;
        {
            char ch = matrix[0].charAt(0);
            if (ch-'A'>=0 && ch-'A'<26) {
                ArrayList<int[]> a = portals.get(ch);
                for (int k=0;k<a.size();k++) {
                    int[] portal_pos = a.get(k);
                    if (visited[portal_pos[0]][portal_pos[1]]) {
                        continue;
                    }
                    q.offer(new int[]{portal_pos[0], portal_pos[1]});
                    visited[portal_pos[0]][portal_pos[1]] = true;
                }
                portals.remove(ch);
            }
        }
        while (!q.isEmpty()) {
            int lev_size = q.size();
            for (int l=0;l<lev_size;l++) {
                int[] pos = q.poll();
                if (pos[0]==rows-1 && pos[1]==cols-1) {
                    return steps;
                }
                for (int d=0;d<dirs.length;d++) {
                    int next_i = pos[0] + dirs[d][0];
                    int next_j = pos[1] + dirs[d][1];
                    if (next_i<0||next_i>=rows||next_j<0||next_j>=cols) { 
                        continue;
                    }
                    char ch = matrix[next_i].charAt(next_j);
                    if (ch =='#' || visited[next_i][next_j]) {
                        continue;
                    }
                    q.offer(new int[]{next_i,next_j});
                    visited[next_i][next_j] = true;
                    if (ch-'A'>=0 && ch-'A'<26) {
                        ArrayList<int[]> a = portals.get(ch);
                        for (int k=0;k<a.size();k++) {
                            int[] portal_pos = a.get(k);
                              if (visited[portal_pos[0]][portal_pos[1]]) {
                                continue;
                            }
                            q.offer(new int[]{portal_pos[0], portal_pos[1]});
                            visited[portal_pos[0]][portal_pos[1]] = true;
                        }
                        portals.remove(ch);
                    }
                }
            }
            steps++;
        }
        return -1;
    }

    public static void main(String[] args) {
        test1();
    }

    public static void test1() {
        String[] matrix = {
            "A..",
            ".A.",
            "..."
        };
        test3 sol = new test3();
        int output = sol.minMoves(matrix);
        boolean passed = output == 2;
        System.out.println("test1:" + (passed ? "passed" : "failed"));
    }
}
