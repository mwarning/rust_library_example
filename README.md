# Rust example library

This is an example library written in Rust to be used via a Foreign Function Interface (FFI).
Included is a C program that uses the library. 

Status: works


Build Rust library (called foo):
```
cd foo
cargo build
```

Build C program (called main):
```
cd ..
gcc main.c -lfoo -L./foo/target/debug/ -omain
```

Run C program that uses the Rust library:
```
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:./foo/target/debug
./main
```

Useful Links:
* http://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/
* http://jakegoulding.com/rust-ffi-omnibus/objects/
* https://michael-f-bryan.github.io/rust-ffi-guide/overview.html
