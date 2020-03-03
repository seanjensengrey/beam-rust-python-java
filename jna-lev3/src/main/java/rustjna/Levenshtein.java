package rustjna;

import com.sun.jna.Library;
import com.sun.jna.Native;


public class Levenshtein {
    public interface CLevenshtein extends Library {
        CLevenshtein INSTANCE = 
            (CLevenshtein) Native.loadLibrary("lev3",CLevenshtein.class);

        int levenshtein(String a, String b);
    }

    public static void main(String[] args) {
        System.out.println("levenshtein('boo','bar') = " 
            + CLevenshtein.INSTANCE.levenshtein("boo","bar"));
    }
}
