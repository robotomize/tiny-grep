# Tiny grep

The tiny-grep is a simple, yet powerful command-line utility written in Rust that filters and highlights specified 
patterns 
in text input from stdin.

## Why
This is a tutorial project that can only read standard input.

## Features
* Pattern Matching: Search for and highlight specified patterns within the text input from stdin.
* Color Highlighting: Uses the colored library to highlight matching patterns in red for easy identification.
* Multiple Pattern Support: Allows specifying multiple patterns as arguments for simultaneous search and highlight.
* Line Numbering: Outputs the line number alongside the original text, aiding in locating the highlighted patterns 
  within the context.

## Requirements
To use this utility, you need to have Rust installed on your system. This project also depends on the colored crate for text coloring functionality.

## Install
```shell
git clone https://github.com/robotomize/tiny-grep.git
cd tinygrep
```

## Usage
To use the grp, simply pass the patterns you want to highlight as arguments when running the program. You can specify 
multiple patterns separated by spaces:
```shell
cargo run -- pattern1 pattern2 pattern3
cargo build
cat main.rs|./tiny-grep stdin
```

## License
This project is licensed under the MIT License - see the LICENSE.md file for details.
