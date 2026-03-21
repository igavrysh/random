public class leetcode73 {
    public void setZeroes(int[][] matrix) {
        int n = matrix.length;
        int m = matrix[0].length;

        for (int i=0;i<n;i++) {
            for (int j=0;j<m;j++) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for (int i=0;i<n;i++) {
            for (int j=0;j<m;j++) {
                System.out.print(matrix[i][j]+",");
            }
            System.out.println("");
        }

        for (int i=0;i<n;i++) {
            for (int j=0;j<m;j++) {
                if (matrix[i][0] == 0) {
                    matrix[i][j] = 0;
                } else if (matrix[0][j]==0) {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    public static void main(String[] args) {
        leetcode73 sol = new leetcode73();
        int[][] matrix = {{0,1,2,0},{3,4,5,2},{1,3,1,5}};
        sol.setZeroes(matrix);
    }
}