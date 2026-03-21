public class LinearSearchTest {

    private static final int SIZE = 280000000;
    private static final int NUM_THREADS = 10;

    private static final Object mtx = new Object();
    private static volatile int foundIndex = -1;

    private static void LinearSearch(int threadId, int[] arr, int key) {
        int chunkSize = arr.length / NUM_THREADS;
        int start = threadId * chunkSize;
        int end = (threadId == NUM_THREADS - 1) ? arr.length : start + chunkSize;

        for (int i = start; i < end; ++i) {
            synchronized (mtx) {
                if (foundIndex != -1) {
                    break;
                }
            }

            if (arr[i] == key) {
                synchronized (mtx) {
                    System.out.println("thread " + threadId + " found");
                    if (foundIndex == -1) {
                        foundIndex = i;
                        break;
                    }
                }
            }
        }
    }

    public static void main(String[] args) {
        int[] arr = new int[SIZE];
        for (int i = 0; i< SIZE; ++i) {
            arr[i] = (int) (Math.random() * 10000);
        }

        Thread[] threads = new Thread[NUM_THREADS];

        int key = 11129;
        for (int i = 0; i < NUM_THREADS; ++i) {
            final int threadId = i;
            threads[i] = new Thread(() -> LinearSearch(threadId, arr, key));
            threads[i].start();
        }

        for (Thread thread : threads) {
            try {
                thread.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        if (foundIndex == -1) {
            System.out.println("Element not found in the array.");
        } else {
            System.out.println("Element found at index: " + foundIndex);
        }
    }
}
