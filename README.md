# Current crate

This repository provides FFI bindings to the bwapi-c library. You probably looking for bwapi-rs crate.

# About project

The whole project is an attempt to create idiomatic Rust bindings to the [BWAPI library](http://bwapi.github.io/).

The main goal is to provide convenient API which will someday allow writing custom bots for the legendary Starcraft/Broodwar game in the [Rust language](https://www.rust-lang.org/). Please note however, that this project is absolutely nothing to do with Starcraft 2.

As you've probably noticed, the project is in it's very early days of development. Actually, we have no idea how it would work. The future will tell.

# Contribution

Ideas and/or contributions are very welcome. Please feel free contacting us by email or using Issues.

# Troubleshooting

1. When openssl-sys build falls with the error:
  ```
  ~/a/bwapi-sys ❯❯❯ cargo build
       Compiling openssl-sys v0.9.15
    error: failed to run custom build command for `openssl-sys v0.9.15`
    process didn't exit successfully: `~/a/bwapi-sys/target/debug/build/openssl-sys-169fec58669c2940/build-script-build` (exit code: 101)
  ```
  - Under Linux you need to install dev version of openssl:
    ```
    ~/a/bwapi-sys ❯❯❯ sudo apt install libssl-dev 
    ```
  - Under Windows you need to manually install OpenSSL and 
    follow [this answer on StackOverflow](https://stackoverflow.com/a/32208817/5066426)

2. `miniz-sys` cannot be built (under Windows)
  ```
    error: failed to run custom build command for `miniz-sys v0.1.9`
    process didn't exit successfully: 
    `C:\bwapi-sys\target\debug\build\miniz-sys-72bb69db49bc9e39\build-script-build`
    (exit code: 101)
  ```
  - You need _MinGW_ installed, it is insufficient 
    to take `gcc.exe` and `ar.exe` from Rust installation. So
    ```
    C:\>choco install mingw
    ```
    and `set PATH=%PATH%;C:\tools\mingw32\bin`

3. Panic during the build
  ```
  thread 'main' panicked at '
  failed to execute command: No such file or directory (os error 2)
  is `cmake` not installed?
  ```
  The message suggests that you need to install `cmake` and it 
  should be found in the path:
  ```
  ❯❯❯ cmake --version
  cmake version 3.7.2
  ```