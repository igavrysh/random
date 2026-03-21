public class CondVarSleepingTest {
    private static final Object mtx = new Object();
    private static int sharedNumber;
    private static boolean ready = false;

    public static void producer() {
        synchronized (mtx) {
            sharedNumber = 42; // producing number
            ready = true;
            System.out.println("Producer has produced the number: " + sharedNumber);
            mtx.notify();
        }
    }

    public static void consumer() {
        synchronized (mtx) {
            while (!ready) {
                try {
                    mtx.wait();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    System.out.println("Consumer thread was interrupted");
                }
            }

            System.out.println("Consumer has consumed the number: " + sharedNumber);
        }
    }

    public static void main(String[] args) {
        Thread producerThread = new Thread(CondVarSleepingTest::producer);
        Thread consumerThread = new Thread(CondVarSleepingTest::consumer);

        producerThread.start();
        consumerThread.start();

        try {
            producerThread.join();
            consumerThread.join();
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            System.out.println("Main thread was interrupted.");
        }
    }
}
