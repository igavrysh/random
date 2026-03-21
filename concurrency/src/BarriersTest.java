import java.util.concurrent.CyclicBarrier;

public class BarriersTest {
    static CyclicBarrier barrier = new CyclicBarrier(2, () ->
            System.out.println("All threads have reached the barrier. Continue execution.")
    );

    public static void main(String[] args) throws InterruptedException {
        Thread t1 = new Thread(BarriersTest::work);
        //t1.setDaemon(true);
        Thread t2 = new Thread(BarriersTest::work);
        //t2.setDaemon(true);

        t1.start();
        t2.start();
        Thread.sleep(5000);

    }

    static void work() {
        System.out.println("Thread " + Thread.currentThread().getName() + " is waiting at the barrier");

        try {
            Thread.sleep(500);
            barrier.await();
            System.out.println("Thread " + Thread.currentThread().getName() + " is released");
        } catch (Exception e) {
        }
    }
}

