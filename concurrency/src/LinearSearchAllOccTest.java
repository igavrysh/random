import java.util.List;
import java.util.ArrayList;
import java.util.Random;

public class LinearSearchAllOccTest {

    private static final int SIZE = 280000000;
    private static final int NUM_THREADS = 10;

    // Mutex for controlling access to foundPlaces
    private static final Object lockObj = new Object();
    private static List<Integer> foundPlaces = new ArrayList<>();

    private static void linearSearch(int threadId, int[] arr, int key) {
        int chunkSize = arr.length / NUM_THREADS;
        int start = threadId * chunkSize;
        int end = (threadId == NUM_THREADS - 1) ? arr.length : start + chunkSize;

        for (int i = start; i < end; ++i) {
            if (arr[i] == key) {
                synchronized (lockObj) { // lock the modifying foundPlaces
                    System.out.println("thread " + threadId + " found");
                    foundPlaces.add(i);  // append the index to foundPlaces
                }
            }
        }
    }

    public static void main(String[] args) {
        // create an array and fill it with random number between 0 and 99
        int[] arr = new int[SIZE];
        Random random = new Random();
        for (int i = 0; i< SIZE; ++i) {
            arr[i] = random.nextInt(100);
        }

        List<Thread> threads = new ArrayList<>(); // list to hold all threads
        int key = 42; // element to find

        // start the threads
        for (int i = 0; i < NUM_THREADS; ++i) {
            final int threadId = i;
            Thread thread = new Thread(() -> linearSearch(threadId, arr, key));
            threads.add(thread);
            thread.start();
        }

        // join the threads with the main thread
        for (Thread thread : threads) {
            try {
                thread.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        // display the result
        if (foundPlaces.isEmpty()) {
            System.out.println("Element not found in the array.");
        } else {
            System.out.println("Element found at indexes");
            synchronized (lockObj) { // Lock when reading from found places
                for (int index : foundPlaces) {
                    System.out.print(index + " ");
                }
            }
            System.out.println();
        }
    }
}
