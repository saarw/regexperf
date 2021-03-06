import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.regex.Pattern;

public class Regexperf {
    public static void main(String[] args) {
        try {
            List<String> lines = Files.readAllLines(Paths.get(args[0]));
            Pattern p = Pattern.compile("^[a-zA-Z0-9_\\s\\r\\n\\t]*$");
            int matched = 0;
            long startTime = 0;
            int lineCount = lines.size();
            int total = lineCount * 20;
            for (int i = 0; i < total; i++) {
                if (i == lineCount) { // Give the JIT a chance to optimize before timing
                    startTime = System.nanoTime();
                }
                if (p.matcher(lines.get(i % lineCount)).matches()) {
                    matched++;
                }
            }
            long time = System.nanoTime() - startTime;
            System.out.format("%s out of %s lines matched, timing %s ms (%s ns per match)\n",
                    matched, total, time / 1000000, (time / (total - lineCount)));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
