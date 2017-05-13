# Regex testing
Test programs to compare performance of Rust and Java regexes without caching information between between matches. Accomplishes this by creating a new matcher for every match in Java and cloning a new Regex for every match in Rust.

The program reads a text file into memory and runs through the file 20 times with a regex that checks whether each line only contains certain characters. It only starts measuring time after the first pass to give Java's Hotspot a chance to optimize the code and better simulate a long-running application.

## How to run
- Make sure Java and Rust are installed
- Download a large text file, for instance ```http://norvig.com/big.txt```
- Compile Java, inside regexperf_java ```javac -d ./bin src/Regexperf.java```
- Run Java, inside regexperf_java ```java -cp ./bin Regexperf ~/Downloads/big.txt```
- Compile Rust, inside regexperf_rs ```cargo build --release```
- Run Rust, inside regexperf_rs ```./target/release/regexperf_rs ~/Downloads/big.txt```

## Results on OSX (2-core Macbook Pro)
Java ```864340 out of 2569140 lines matched, timing 1636 ms (670 ns per match)```   
Rust ```864340 out of 2569140 lines matched, timing 11677 ms (4784 ns per match)```   
    
Later added capability to specify batch sizes to the Rest test   
Rust with batch sizes specified   
```864340 out of 2569140 lines matched, with batch size 10, timing 5498 ms (2252 ns per match)```   
```864340 out of 2569140 lines matched, with batch size 100, timing 4073 ms (1668 ns per match)```

## Results on Windows 10 (Dell XPS 15 9550)
Java ```864340 out of 2569140 lines matched, timing 1472 ms (603 ns per match)```   
Rust ```864340 out of 2569140 lines matched, timing 13585 ms (5566 ns per match)```
