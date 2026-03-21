public class CondVarBusyWaitingTest {

    private static final Object mtx = new Object();
    private static int sharedNumber;
    private static boolean ready = false;

    private static void producer() {
        synchronized (mtx) {
            sharedNumber = 43;
            ready = true;
            System.out.println("Producer has produced the number: " + sharedNumber);
        }
    }

    private static void consumer() {
        while (true) {
            synchronized (mtx) {
                if (ready) {
                    System.out.println("Consumer has consumed the number: " + sharedNumber);
                    break;
                }
            }
            try {
                Thread.sleep(1); // sleep for short time
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                System.out.println("Thread was interrupted");
            }
        }
    }

    public static void main(String[] args) {
        Thread producerThread = new Thread(CondVarBusyWaitingTest::producer);
        Thread consumerThread = new Thread(CondVarBusyWaitingTest::consumer);

        consumerThread.start();

        producerThread.start();

        try {
            producerThread.join();
            consumerThread.join();
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            System.out.println("Main thread was interrupted");
        }
    }
}
