import java.util.PriorityQueue;

class leetcode0767 {
    public String reorganizeString(String s) {
        int[] fq = new int[26];
        int n = s.length();
        for (int i=0;i<n;i++) {
            fq[s.charAt(i)-'a']++;
        }
        StringBuilder sb = new StringBuilder();
        PriorityQueue<int[]> pq = new PriorityQueue<>((int[] p1, int[] p2) -> {
            return -1*Integer.compare(p1[1],p2[1]);
        });
        for (int i=0;i<26;i++) {
            if (fq[i]==0) { continue; }
            pq.offer(new int[]{i,fq[i]});
        }
        int[] prev = null;
        for (int i=0;i<n;i++) {
            if (pq.isEmpty()) {
                return "";
            }
            int[] curr = pq.poll();
            System.out.println("peeked: " + curr[0] + ";"+curr[1]);
            sb.append((char)(curr[0]+'a'));
            curr[1]--;
            if (prev != null && prev[1]!=0) {
                System.out.println("offering: i:"+prev[0]+";fq"+prev[1]);
                pq.offer(prev);
            }
            prev = curr;
        }
        return sb.toString();
    }
    public static void main(String[] args) {
        test1();
    }

    public static void test1() {
        String s = "aab";
        leetcode0767 sol = new leetcode0767();
        String output = sol.reorganizeString(s);
        int t = 1;
    }
}