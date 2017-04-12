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

On Linux, install these through your favorite package management tool, or
through [libquantum's website](http://www.libquantum.de/).

For example, on Ubuntu one can install libquantum through the command

```
sudo apt-get install libquantum-dev
```

At compile-time, the library uses `rust-bindgen` to generate FFI-level
bindings from the libquantum header file, which it expects to find in
`/usr/include/quantum.h`

Unfortunately, no other operating systems are officially supported because
I have no way to test on them. Presumably, however, libquantum works on
Mac OS X, and you can install it via homebrew

```
brew install libquantum
```

You may have to change the header file's path in `build.rs`, but otherwise I
don't see why it shouldn't work. A similar method should work for getting it
set up on FreeBSD and other operating systems supported by libquantum.

The hard-coded header file path should be fixed soon.

### Installation

If you're using Cargo to manage your project, you can download through [crates.io](http://crates.io).

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

### Contributing

Just submit a pull request, any help in getting these bindings general and
complete is welcome. Some rules, though:

* Any new code must be well-documented, whether or not it appears in the
  public interface.
* If you want to change an existing public interface, give me a good reason.

