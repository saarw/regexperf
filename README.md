# Regex testing
Test programs to compare performance of Rust and Java regexes without caching information between between matches. Accomplishes this by creating a new matcher for every match in Java and cloning a new Regex for every match in Rust.

The program reads a text file into memory and runs through the file 10 times with a regex that checks whether each line only contains certain characters. It only starts measuring time after the first pass to give Java's Hotspot a chance to optimize the code.

## How to run
- Make sure Java and Rust are installed
- Download a large text file, for instance ```http://norvig.com/big.txt```
- Compile Java, inside regexperf_java ```javac -d ./bin src/Regexperf.java```
- Run Java, inside regexperf_java ```java -cp ./bin Regexperf ~/Downloads/big.txt```
- Compile Rust, inside regexperf_rs ```cargo build --release```
- Run Rust, inside regexperf_rs ```./target/release/regexperf_rs ~/Downloads/big.txt```

## Results on 2-core Macbook Pro
Java ```432170 out of 1284570 lines matched, timing 770 millis```   
Rust ```432170 out of 1284570 lines matched, timing 403 millis```

