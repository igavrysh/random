import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class LinearSearchAllIdxOccTest {
    private static final int SIZE = 1000;
    private static final int NUM_THREADS = 10;

    private static final List<Integer> foundIndices = new ArrayList<>(); // shared list to store
    private static int occurencesCount = 0; // shared variable to store the count of occurences

    // Mutex for controlling access to foundPlaces
    private static final Object indicesLock = new Object(); // lock for sync access to found indices
    private static final Object countLock = new Object(); // lock for sync access to occurences count

    // function executed by each thread to search for indices and occurences
    private static void searchIndicesOccurences(int threadId, int[] arr, int key) {
        int chunkSize = arr.length / NUM_THREADS;
        int start = threadId * chunkSize;
        int end = (threadId == NUM_THREADS - 1) ? arr.length : start + chunkSize;

        List<Integer> localIndices = new ArrayList<>();
        int localCount = 0;

        for (int i = start; i < end; ++i) {
            if (arr[i] == key) {
                localIndices.add(i);
                localCount++;
            }
        }

        if (!localIndices.isEmpty()) {
            synchronized (indicesLock) {
                foundIndices.addAll(localIndices);
            }
        }

        if (localCount > 0) {
            occurencesCount += localCount;
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
            Thread thread = new Thread(() -> searchIndicesOccurences(threadId, arr, key));
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
        if (foundIndices.isEmpty()) {
            System.out.println("Element not found in the array.");
        } else {
            System.out.print("Element found " + occurencesCount + " times at indexes: ");
            for (int index : foundIndices) {
                System.out.print(index + " ");
            }
            System.out.println();
        }
    }
}
