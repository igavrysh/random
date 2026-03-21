import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class FizzBuzzMultiThrTest {

    private final int n; // total number of sequence
    private int current; // current number of being prcessed
    private final Lock lock = new ReentrantLock(); // lock for sync
    private final Condition cond = lock.newCondition(); // cond variable for coord b/w threads


    public FizzBuzzMultiThrTest(int n) {
        this.n = n;
        this.current = 1;
    }

    public void fizz() throws InterruptedException {
        while (true) {
            lock.lock();
            try {
                while (current <= n && !(current % 3 == 0 && current % 5 != 0)) {
                    cond.await();
                }
                if (current > n) break;
                System.out.println("fizz");
                current++;
                cond.signalAll();
            } finally {
                lock.unlock();
            }
        }
    }

    public void buzz() throws InterruptedException {
        while (true) {
            lock.lock();
            try {
                while (current <= n && !(current % 5 == 0 && current % 3 != 0)) {
                    cond.await();
                }
                if (current > n) {
                    break;
                }
                System.out.println("buzz");
                current++;
                cond.signalAll();
            } finally {
                lock.unlock();
            }
        }
    }

    public void fizzbuzz() throws InterruptedException {
        while (true) {
            lock.lock();
            try {
                while (current <= n && !(current % 15 == 0)) {
                    cond.await();
                }
                if (current > n) break;
                System.out.println("fizzbuzz");
                current++;
                cond.signalAll();
            } finally {
                lock.unlock();
            }
        }
    }

    public void number() throws InterruptedException {
        while (true) {
            lock.lock();
            try {
                while (current <= n && !(current % 3 != 0 && current % 5 != 0)) {
                    cond.await();
                }
                if (current > n) break;
                System.out.println(current);
                current++;
                cond.signalAll();
            } finally {
                lock.unlock();
            }
        }
    }

    public static void main(String[] args) {
        int n = 10;
        FizzBuzzMultiThrTest fb = new FizzBuzzMultiThrTest(n);

        Thread t1 = new Thread(() -> {
            try {
                fb.fizz();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        });
        Thread t2 = new Thread(() -> {
            try {
                fb.buzz();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        });
        Thread t3 = new Thread(() -> {
            try {
                fb.fizzbuzz();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        });
        Thread t4 = new Thread(() -> {
            try {
                fb.number();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        });

        t1.start();
        t2.start();
        t3.start();
        t4.start();
    }
}
