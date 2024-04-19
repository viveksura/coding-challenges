# [WordCount tool in Rust](https://codingchallenges.fyi/challenges/challenge-wc)

### Dependencies
Rust

### Build instructions
`cargo build` to build library
Executable `target/debug/ccwc` will be created 

### Running Instructions

`target/debug/ccwc -c <filepath>` 

`target/debug/ccwc -w <filepath>`

`target/debug/ccwc -l <filepath>`

`target/debug/ccwc <filepath>`

`cat <filepath> | target/debug/ccwc -c`

`cat <filepath> | target/debug/ccwc -w`

`cat <filepath> | target/debug/ccwc -l`

`cat <filepath> | target/debug/ccwc`