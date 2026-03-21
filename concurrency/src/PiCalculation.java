import java.util.Random;

public class PiCalculation {

    private static final int NUM_THREADS = 4;
    private static final int NUM_OF_TOSSES = 1000000;
    private static int[] results = new int[NUM_THREADS];

    public static void main(String[] args) throws InterruptedException {
        Thread[] threads = new Thread[NUM_THREADS];
        for (int i=0;i<NUM_THREADS;i++) {
            final int threadId = i;
            threads[i] = new Thread(() -> {
                Random random = new Random();
                int start = (threadId * NUM_OF_TOSSES) / NUM_THREADS;
                int end = ((threadId + 1) * NUM_OF_TOSSES) / NUM_THREADS;
                int count_in_circle = 0;

                for (int j=start;j<end;j++) {
                    double x = random.nextDouble() * 2 - 1; // random in range [-1,1]
                    double y = random.nextDouble() * 2 - 1; // random in range [-1,1]
                    if (x*x + y*y <= 1) {  // if point is inside the circle
                        count_in_circle++;
                    }
                }
                results[threadId] = count_in_circle;
            });
            threads[i].start();
        }
        for (Thread thread : threads) {
            thread.join();
        }

        // Compute final estimate of pi
        int total_inside = 0;
        for (int result : results) {
            total_inside += result;
        }
        double pi_estimate = (4.0 * total_inside) / NUM_OF_TOSSES;
        System.out.println("PI = " + pi_estimate);
    }
}
