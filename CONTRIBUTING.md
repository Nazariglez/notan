# Contributing to Notan

## Linux
> TODO

## MacOS
> TODO

## Windows (MSVC)

To be able to compile the Notan examples you will need to have the following pre-requirements in your system:

* [CMake](https://cmake.org/download/)
* [Python](https://www.python.org/downloads/)
* [Ninja](https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages)

If you want to use the wasm32 target you will need to lauch some additional commands for it:
````cmd
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli --version 0.2.87
cargo install wasm-opt
````

After this you will be able to use the PowerShell scripts in the `scripts` folder to compile the examples or the doc.
