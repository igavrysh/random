class Solution {
    public int[] constructDistancedSequence(int n) {
        int m = (n-1)*2+1;
        int[][] output = new int[1][m];
        int[] cumul = new int[m];
        for (int i = 0; i < cumul.length; i++) {
            cumul[i] = 1;
            bt(2, n, cumul, output);
            cumul[i] = 0;
        }
        return output[0];
    }
    public void bt(int num, int n, int[] cumul, int[][] output) {
        if (num > n) {
            boolean greater = false;
            for (int i = 0; i < cumul.length; i++) {
                if (cumul[i] > output[0][i]) {
                    greater = true;
                    break;
                }
            }
            if (greater) {
                for (int i = 0; i < cumul.length; i++) {
                    output[0][i] = cumul[i];
                }
            }
            return;
        }
        for (int i = 0; i < cumul.length; i++) {
            if (cumul[i] == 0 && i+num < cumul.length && cumul[i+num] == 0) {
                cumul[i] = num;
                cumul[i+num] = num;
                bt(num+1, n, cumul, output);
                cumul[i] = 0;
                cumul[i+num] = 0;
            }
        }
    }

    public static void main(String[] args) {
        int n = 3;
        Solution sol = new Solution();
        int[] output = sol.constructDistancedSequence(n);
        int t = 1;
    }
}