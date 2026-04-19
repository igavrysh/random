import java.util.ArrayList;

class lc2981 {
    public int maximumLength(String s) {
        @SuppressWarnings("unchecked")
        ArrayList<int[]>[] a = new ArrayList[26];
        char prev_ch = s.charAt(0);
        char seq = 1;
        for (int i = 1; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (ch == prev_ch) {
                seq++;
            } else {
                ArrayList<int[]> arr = a[prev_ch-'a'];
                if (arr == null) {
                    arr = new ArrayList<int[]>();
                    a[prev_ch-'a'] = arr;
                }
                int idx = bs(arr, seq);
                if (idx < arr.size() && arr.get(idx)[0] == seq) {
                    arr.get(idx)[1]++;
                } else {
                    arr.add(idx, new int[]{seq, 1});
                }
                seq = 1;
            }
            prev_ch = ch;
        }
        ArrayList<int[]> arr = a[prev_ch-'a'];
        if (arr == null) {
            arr = new ArrayList<>();
            a[prev_ch-'a'] = arr; 
        }
        int idx = bs(arr, seq);
        if (idx < arr.size() && arr.get(idx)[0] == seq) {
            arr.get(idx)[1]++;
        } else {
            arr.add(idx, new int[]{seq, 1});
        }
        int longest = -1;
        for (int i = 0; i < 26; i++) {
            arr = a[i];
            if (arr == null) {
                continue;
            }            
            for (int j = arr.size()-1; j >= 0; j--) {
                int[] pair = arr.get(j);
                int[] prev_pair = null;
                if (j > 0) {
                    prev_pair = arr.get(j-1);
                }
                if (pair[1] >= 3) {
                    longest = Math.max(longest, pair[0]);
                }

                if (pair[1] >= 2 && pair[0] >= 2) {
                    longest = Math.max(longest, pair[0]-1);
                }
                
                if (prev_pair != null && prev_pair[1]+(pair[0]-prev_pair[0]+1)*pair[1]>=3) {
                    longest = Math.max(longest, prev_pair[0]);
                }
                
                if (pair[0]>=3) {
                    longest = Math.max(longest, pair[0]-2);
                }
            }
        }
        return longest;
    }
    private int bs(ArrayList<int[]> a, int t_len) {
        int bad = -1;
        int good = a.size();
        while ((good - bad) > 1) {
            int m = bad + (good - bad)/2;
            int[] middle = a.get(m);
            if (middle[0] == t_len) {
                return m;
            }
            if (middle[0] < t_len) {
                bad = m;
            } else {
                good = m;
            }
        }
        return good;
    }
    public static void main(String[] args) {
        String s = "ceeeeeeeeeeeebmmmfffeeeeeeeeeeeewww";
        lc2981 sol = new lc2981();
        int output = sol.maximumLength(s);
        boolean passed = output == 3;
        System.out.println(passed ? "passed" : "failed");
    }
    
}
