# flush-lang

flush-lang is an interpreted programming language written in Rust.

![stars](https://img.shields.io/github/stars/flush-lang/flush?style=for-the-badge)
![forks](https://img.shields.io/github/forks/flush-lang/flush?color=FBA400&style=for-the-badge)

## Installation

You need [`git`](https://git-scm.com/) (or you can download the zip project) and [`cargo`](https://doc.rust-lang.org/cargo/) (for the build).

```sh
git clone https://github.com/flush-lang/flush
cd flush
cargo test
cargo install --path .
```

## Version

Show the current flush's version by using: `flush --version` or `flush -V`.

## Usage

Run a file with: `flush <path/to/file>`.

## Examples

You can find all examples [here](./examples/).

### Hello, World!

```scala
def main() {
    putStrLn("Hello, World!")
}
```

### Factorial

```scala
def fac(n) {
    if (n <= 1) {
        return 1
    }

    return n * fac(n - 1)
}
```
