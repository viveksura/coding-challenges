# [cut tool in Rust](https://codingchallenges.fyi/challenges/challenge-cut/)

### Dependencies
Rust

### Build instructions
`cargo build` to build library
Executable `target/debug/cccut` will be created 

### Running Instructions

`target/debug/cccut -f "1 2" -d , <filepath>` 

`target/debug/cccut -f "1, 2" -d , <filepath>` 

`cat <filepath> | target/debug/cccut -f "1, 2"`