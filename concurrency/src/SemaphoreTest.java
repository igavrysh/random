import java.util.concurrent.Semaphore;
import java.util.concurrent.atomic.AtomicInteger;

public class SemaphoreTest {

    private static final AtomicInteger counter = new AtomicInteger(0);

    private static final Semaphore semaphore = new Semaphore(2);

    private static final int TARGET_VALUE = 5_000;

    public static void main(String[] arg) {
        long startTime = System.currentTimeMillis();

        Thread[] workers = new Thread[10];
        for (int i =0; i<workers.length; i++) {
            workers[i] = new Thread(SemaphoreTest::worker);
            workers[i].start();
        }

        for (Thread worker : workers) {
            try {
                worker.join();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                System.out.println("Thread was interrupted");
            }
        }

        long endTime = System.currentTimeMillis();
        System.out.println("Time take: " + (endTime - startTime) / 1000.0 + " seconds");
    }

    private static void worker() {
        while (true) {
            try {
                semaphore.acquire();
                if (counter.get() >= TARGET_VALUE) {
                    break;
                }
                counter.incrementAndGet();
                Thread.sleep(1);
            } catch (InterruptedException ex) {
                Thread.currentThread().interrupt();
                System.out.println("Thread was interrupted");
            } finally {
                semaphore.release();
            }
        }
    }
}
