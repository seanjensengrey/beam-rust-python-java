# beam-rust-python-java

This currently shows a couple methods of interfacing Rust code with Python and Java.

* Python
  * via [PyO3](https://github.com/PyO3/PyO3)
  * [CFFI](https://cffi.readthedocs.io/en/latest/)
* Java
  * Java Native Access [JNA](https://github.com/java-native-access/jna)

The example code integrates a [Levenshtein Distance](https://en.wikipedia.org/wiki/Levenshtein_distance) routine from the following libraries

* https://crates.io/crates/levenshtein
* https://crates.io/crates/strsim

Slides,  https://docs.google.com/presentation/d/18vT1ebpMWq2yKIAEAslnCM6Ln3VFG1XTwMG2IIMBFog/view

## env

```
# deactive any current envs
deactivate
python3 -m venv lev.env
. lev.env/bin/activate
pip install -U pip
pip install -r requirements.txt
```

## build, pyo3 example

```
cd lev10
rustup override set nightly
cargo test
maturin build --interpreter `which python3`
```

## build, jna example

```
cd jna-lev3
cargo build
mvn compile
ln -s target/debug/liblev3.dylib .
```

### test, jna

```
mvn exec:java -Dexec.mainClass="rustjna.Levenshtein"
```

output

```
[INFO] Scanning for projects...
[INFO] 
[INFO] ----------------------------< rustjna:test >----------------------------
[INFO] Building levenshtein 1.0-SNAPSHOT
[INFO] --------------------------------[ jar ]---------------------------------
[INFO] 
[INFO] --- exec-maven-plugin:1.6.0:java (default-cli) @ test ---
levenshtein('boo','bar') = 2
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time: 1.136 s
[INFO] Finished at: 2020-03-02T17:18:59-08:00
[INFO] ------------------------------------------------------------------------
```


