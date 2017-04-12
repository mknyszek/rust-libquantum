# rust-libquantum

Bindings for libquantum in Rust

## Overview

rust-libquantum is a library that provides safe bindings to the libquantum
C library, a quantum simulator.

Like libquantum, rust-libquantum is licensed under GPL-3.0 as it links
dynamically against libquantum.

## Requirements

### Rust

This library targets the newest stable version of Rust.

### libquantum Development Library

Note: the below instructions should work, but have only been tested for Linux
since I don't actually have access to development machines for any other
systems supported by both Rust and libquantum.

#### Linux

Install libquantum through your favorite package management tool, or
through [libquantum's website](http://www.libquantum.de/).

For example, on Ubuntu one can install libquantum through the command

```
sudo apt-get install libquantum-dev
```

#### Mac OS X

Presumably libquantum works on Mac OS X, and you can install it via homebrew

```
brew install libquantum
```

## Installation

If you're using Cargo to manage your project, you can install through
[crates.io](http://crates.io).

```
[dependencies]
libquantum = "0.1"
```

You can also pull from GitHub to use the latest version.

```
[dependencies.libquantum]
git = "https://github.com/mknyszek/rust-libquantum"
```

Finally, you can also just clone this repository and compile with `cargo build`

## Troubleshooting

If for some reason the build script cannot find `quantum.h` on your system, you
can set the `LIBQUANTUM_INCLUDE` environment variable to be the path to
`quantum.h`.

## Contributing

Just submit a pull request, any help in getting these bindings general and
complete is welcome. Some rules, though:

* Any new code must be well-documented, whether or not it appears in the
  public interface.
* If you want to change an existing public interface, give me a good reason.
* Please add tests for any new functionality you create.

