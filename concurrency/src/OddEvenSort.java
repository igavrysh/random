import java.util.Random;
import java.util.concurrent.BrokenBarrierException;
import java.util.concurrent.CyclicBarrier;

public class OddEvenSort {

    private static final int NUM_THREADS = 4; // number of threads to use
    private static int[] array; // array to be sorted

    // barrier to sync threads in each phrase
    private static CyclicBarrier barrier = new CyclicBarrier(NUM_THREADS);

    // func to perform the odd-even sort on a chunk of the arr
    private static void threadSort(int threadId) {
        int n = array.length;
        for (int phase = 0; phase < n; ++phase) {
            // determine the starting index for this thread
            int begin = (threadId % 2 == phase % 2) ? threadId : threadId + 1;

            // perform comparisons and swaps
            for (int i=begin; i+1<n; i+=NUM_THREADS) {
                if (array[i] > array[i+1]) {
                    int tmp = array[i];
                    array[i] = array[i+1];
                    array[i+1] = tmp;
                }
            }

            // wait for all threads to finish this phase
            try {
                barrier.await();
            } catch (InterruptedException | BrokenBarrierException e) {
                e.printStackTrace();
            }
        }
    }

    public static void main(String[] args) {
        // initialize the array with random numbers
        Random rand = new Random();
        int n = 20; // size of the array
        array = new int[n];
        for (int i=0;i<n;i++) {
            array[i] = rand.nextInt(100)+1;
        }

        // output the original array
        System.out.print("original array: ");
        for (int num : array) {
            System.out.print(num + " ");
        }
        System.out.println();

        // create and start threads for sorting
        Thread[] threads = new Thread[NUM_THREADS];
        for (int i=0;i<NUM_THREADS;i++) {
            final int threadId = i;
            threads[i] = new Thread(() -> threadSort(threadId));
            threads[i].start();
        }

        // wait for all threads to complete
        for (Thread thread : threads) {
            try {
                thread.join();
            } catch(InterruptedException e) {
                e.printStackTrace();
            }
        }

        // output the sorted array
        System.out.print("sorted array: ");
        for (int num : array) {
            System.out.print(num + " ");
        }
        System.out.println();
    }
}
