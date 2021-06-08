# Brainrust

## Table of contents
* [General info](#general-info)
* [How to compile](#how-to-compile)
* [Usage](#usage)
* [Specs](#specs)
* [Syntax](#syntax)

## General info
Cross-platform virtual machine for the Brainf*ck language, written in pure Rust.
It reads source code (as standard text) and executes it, outputting to either file or stdout.
It can also accept input from file or command line.

## How to compile
Assuming `rustup` and `cargo` are installed. From root, `cargo build --release`.

## Usage
```
brainrust [options] <source>
```
Options:
* `-i <file>, --input <file>`<br/>
File read as input for the VM.
* `-s <string>, --stream <string>`<br/>
String used inline as input for the VM. Prepended to the input file contents.
* `-o <file>, --output <file>`<br/>
File written as output of the VM. Required, unless '--print' flag is set.
* `-p, --print`<br/>
Prints the output of the VM to stdout. Required, unless '--output <file>' flag is set.
* `-h, --help`<br/>
Prints this information.

## Specs
The VM offers a virtual linear memory, infinite in both directions (as large as the hosting machine supports).
Each entry in the memory is a 8 bit unsigned value (0 ~ 255) with wrapping arithmetic.
When reading input, it will consume one byte from the buffer (or value 0 if empty).

## Syntax
* `>` moves cursor right.
* `<` moves cursor left.
* `+` increases value at cursor.
* `-` decreases value at cursor.
* `.` outputs value at cursor.
* `,` inputs into value at cursor.
* `[` jumps beyond matching `]` if value at cursor is zero.
* `]` loops back to matching `[`.

Every other symbol is ignored when reading source.
