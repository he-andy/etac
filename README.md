# etac

This is my implementation of a compiler for the [Eta](https://www.cs.cornell.edu/courses/cs4120/2023sp/project/language.pdf?1681909895) Programming Language for CS4120. 

## Prerequisites

Before you begin, ensure you have the following prerequisites installed:

- Rust: If you haven't installed Rust, you can do so by following the instructions at [rustup.rs](https://rustup.rs/).

## Getting Started

1. Clone this repository to your local machine:
3. Build the project using Cargo:

    ```bash
    cargo build --release
    ```
    or run the `etac-build` script provided.

## Examples
Example `.eta` files can be located in the `tests` directory. 

## Usage 
Eta source code can be compiled using the following command: 
`etac [OPTIONS] <SOURCE FILES>`

``` 
Arguments: 
  <SOURCE FILES>  Source files to perform analysis on (can specify multiple)

Options: 
  --help         Print help     
  --lex          Generate output from lexical analysis
  --parse        Generate output from syntactic analysis
  --typecheck    Generate output from semantic analysis
  --irgen        Generate intermediate code
  --irrun        Generate and interpret intermediate code
 -D             Place generated diagnostic files in directory relative to path
 -d             Place generated assembly output files in directory relative to path
 -sourcepath    Specify directory where to find input source files
 -libpath       Specify directory where to find library interface files
 -O             Disable Optimizations
 -target        Specify the operating system for which to generate code
 -report-opts   Print out the optimizations that are supported by this compiler
```


