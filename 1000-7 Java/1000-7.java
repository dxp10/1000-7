public class Main {
    public static void main(String[] args) throws InterruptedException {
        int result = 1000;
        while ((result -7) >= 0) {
            result -= 7;
            System.out.println(result);
            Thread.sleep(100);
        }
    }
}