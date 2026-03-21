import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class MinMaxSumTest {
    private static final int DATA_SIZE = 1000;
    private static final int NUM_THREADS = 10;
    private static int[] data = new int[DATA_SIZE];
    private static int[] threadResultsSum = new int[NUM_THREADS];
    private static int[] threadResultsMin = new int[NUM_THREADS];
    private static int[] threadResultsMax = new int[NUM_THREADS];

    private static void threadedSum(int threadId, int start, int end) {
        int sum = 0;
        for (int i=start;i<end;i++) {
            sum += data[i];
        }
        threadResultsSum[threadId] = sum;
    }

    private static void threadedMin(int threadId, int start, int end) {
        int min = Integer.MAX_VALUE;
        for (int i=start;i<end;i++) {
            min = Math.min(min, data[i]);
        }
        threadResultsMin[threadId] = min;
    }

    private static void threadedMax(int threadId, int start, int end) {
        int max = Integer.MIN_VALUE;
        for (int i=start;i<end;i++) {
            max = Math.max(max, data[i]);
        }
        threadResultsMax[threadId] = max;
    }

    public static void main(String[] args) throws InterruptedException {
        // create an array and fill it with random number between 0 and 99
        Random random = new Random();
        for (int i = 0; i < DATA_SIZE; ++i) {
            data[i] = random.nextInt(100);
        }

        Thread[] threads = new Thread[NUM_THREADS * 3]; // list to hold all threads

        // start the threads
        for (int i = 0; i < NUM_THREADS; ++i) {
            final int threadId = i;
            final int start = threadId * (DATA_SIZE / NUM_THREADS);
            final int end = (threadId+1) * (DATA_SIZE / NUM_THREADS);
            threads[threadId] = new Thread(() -> threadedSum(threadId, start, end));
            threads[threadId+NUM_THREADS] = new Thread(() -> threadedMin(threadId, start, end));
            threads[threadId+NUM_THREADS*2] = new Thread(() -> threadedMax(threadId, start, end));
            threads[threadId].start();
            threads[threadId+NUM_THREADS].start();
            threads[threadId+NUM_THREADS*2].start();
        }

        // join the threads with the main thread
        for (Thread thread : threads) {
            thread.join();
        }

        // aggregate results from threads
        int totalSum = 0;
        for (int sum : threadResultsSum) {
            totalSum += sum;
        }

        int min = Integer.MAX_VALUE;
        for (int minResult : threadResultsMin) {
            min = Math.min(min, minResult);
        }

        int max = Integer.MIN_VALUE;
        for (int maxResult : threadResultsMax) {
            max = Math.max(max, maxResult);
        }

        System.out.println("Sum is " + totalSum);
        System.out.println("Min is " + min);
        System.out.println("Max is " + max);
    }
}
