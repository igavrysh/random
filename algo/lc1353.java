import java.util.Arrays;
import java.util.PriorityQueue;;
class lc1353 { 
    public int maxEvents(int[][] events) {
        Arrays.sort(events, (int[] e1, int[] e2) -> {
            if (e1[0]==e2[0]) {
                return Integer.compare(e1[1],e2[1]);
            }
            return Integer.compare(e1[0],e2[0]);
        });
        PriorityQueue<Integer> pq = new PriorityQueue<>((Integer d1, Integer d2) -> {
            return Integer.compare(d1,d2);
        });
        int ptr = 0;
        int attended = 0;
        for (int i=1;i<=100000;i++) {
            while (ptr<events.length && events[ptr][0]<=i) {
                pq.offer(events[ptr][1]);
                ptr++;
            }
            while (!pq.isEmpty() && pq.peek()<i) {
                pq.poll();
            }
            if (pq.size()>0) {
                pq.poll();
                attended++;
            }
            if (pq.size()==0 && ptr<events.length-1) {
                i = events[ptr][0]-1;
            }
        }
        return attended;
    }
    public static void test2() {
        int[][] events = {{1,2},{1,2},{3,3},{1,5},{1,5}};
        lc1353 sol = new lc1353();
        int output = sol.maxEvents(events);
        int exp_output = 5;
        boolean passed = output == exp_output;
        System.out.println("test2: " + (passed ? "passed" : "failed"));
    }
    public static void test1() {
        int[][] events = {{1,2},{2,3},{3,4}};
        lc1353 sol = new lc1353();
        int output = sol.maxEvents(events);
        int exp_output = 3;
        boolean passed = output == exp_output;
        System.out.println("test1: " + (passed ? "passed" : "failed"));
    }
    public static void main(String[] args) {
        test2();
        test1(); 
    }
}