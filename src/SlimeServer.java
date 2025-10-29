import java.util.Random;
import java.util.Scanner;

public class SlimeServer {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        while (sc.hasNext()) {
            if (!sc.hasNextLong()) break;
            long seed = sc.nextLong();
            int chunkX = sc.nextInt();
            int chunkZ = sc.nextInt();

            boolean result = isSlimeChunk(seed, chunkX, chunkZ);
            System.out.println(result ? "1" : "0");
            System.out.flush();
        }
        sc.close();
    }

    public static boolean isSlimeChunk(long worldSeed, int xPosition, int zPosition) {
        Random rnd = new Random(worldSeed
                   + (int) (xPosition * xPosition * 0x4c1906)
                   + (int) (xPosition * 0x5ac0db)
                   + (int) (zPosition * zPosition) * 0x4307a7L
                   + (int) (zPosition * 0x5f24f) ^ 0x3ad8025fL);
        return rnd.nextInt(10) == 0;        
    }

}