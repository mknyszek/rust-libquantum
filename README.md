# rust-libquantum

> Bindings for libquantum in Rust

`rust-libquantum` is a library that provides safe bindings to the libquantum
C library, a quantum simulator.

Like libquantum, rust-libquantum is licensed under GPL-3.0 as it links
dynamically against libquantum.

## Prerequisits

* Rust (install [here](https://www.rustup.rs)).
* Libquantum (can be installed from [source] or through a package manager, for example `sudo apt-get install libquantum-dev` or `brew install libquantum`)

## Installation

```
[dependencies]
libquantum = "0.2"
```

You can also pull from GitHub to use the latest version.

```
[dependencies.libquantum]
git = "https://github.com/mknyszek/rust-libquantum"
```

Finally, you can also just clone this repository and compile with `cargo build`

## Troubleshooting

If for some reason the build script cannot find `quantum.h` on your system, try installing from source

## Contributing

Just submit a pull request, any help in getting these bindings general and
complete is welcome. Some rules, though:

* Any new code must be well-documented, whether or not it appears in the
  public interface.
* If you want to change an existing public interface, give me a good reason.
* Please add tests for any new functionality you create.

