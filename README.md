# Ruzzer

## Fuzzing Technique

*Fuzzing or fuzz testing is an automated software testing technique that involves providing invalid, unexpected, or random data as inputs to a computer program. The program is then monitored for exceptions such as crashes, failing built-in code assertions, or potential memory leaks. Typically, fuzzers are used to test programs that take structured inputs. This structure is specified, e.g., in a file format or protocol and distinguishes valid from invalid input. An effective fuzzer generates semi-valid inputs that are "valid enough" in that they are not directly rejected by the parser, but do create unexpected behaviors deeper in the program and are "invalid enough" to expose corner cases that have not been properly dealt with.*
*Wikipedia*

## Description

Ruzzer creates string mutations and permutes the characters randomly with 13% chance every iteration, and adds 10 random characters every 500 iterations.

## Installation

Local build

```rust
cargo build 
```

Release build

```rust
cargo build --release
```

## Execution

Mac/Linux

```bash
./target/debug/ruzzer --iter <iter> --prng_seed <prng>
```

Windows

```bash
ruzzer.exe --iter <iter> --prng_seed <prng>
```
