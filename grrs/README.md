# PoC 1: CLI Tool - grrs (Grep Clone)

## Description:
The goal of this Proof of Concept (PoC) is to implement a simple command-line interface (CLI) tool called grrs, inspired by the Unix grep command. The tool will search for a specific pattern within a text file and print the lines that match the pattern.

### A typical invocation of our CLI tool will look like this:
```sh
$ grrs pattern path/to/file 

$ grrs foo bar.txt

```
> The first argument is the pattern we want to search for, and the second argument is the path to the file we want to search in.

***Libraries Used:***
- [std::env](https://doc.rust-lang.org/std/env/index.html)
- [clap](https://docs.rs/clap/2.33.3/clap/)
- [std::fs](https://doc.rust-lang.org/std/fs/index.html)