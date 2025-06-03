# pylon-cxx-rs

[![Crates.io](https://img.shields.io/crates/v/pylon-cxx.svg)](https://crates.io/crates/pylon-cxx)

Rust wrapper of the Pylon libraries for Basler cameras using [CXX](https://crates.io/crates/cxx).

## Platform support

Windows, linux, and macOS are all tested.

## async stream with tokio

Enable async stream support using tokio with the cargo feature `stream`. 

## Building

This crate expects to find the Pylon developer kit at the usual install
location. Build with normal rust commands. For example, to run the `grab` example:

    cargo run --example grab

### On macOS

On macOS, building should work, but it is required update your environment
variables to run the produced binaries:

    export DYLD_LIBRARY_PATH="/Library/Frameworks/pylon.framework/Versions/A/Libraries/"

I have seen articles like
https://jorgen.tjer.no/post/2014/05/20/dt-rpath-ld-and-at-rpath-dyld/ which
suggest a way to avoid this issue.

## Camera emulation

See [Basler's documentation](https://docs.baslerweb.com/camera-emulation.html). This can
simulate different frame rates, failures, etc.

```text
# on bash (e.g. linux)
export PYLON_CAMEMU=2
```

```text
# in Windows Powershell
$Env:PYLON_CAMEMU=2
```

## Code of conduct

Anyone who interacts with this software in any space, including but not limited
to this GitHub repository, must follow our [code of
conduct](code_of_conduct.md).

## License

This crate is Copyright (C) 2020 Andrew Straw <strawman@astraw.com>.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
http://opensource.org/licenses/MIT>, at your option. This file may not be
copied, modified, or distributed except according to those terms.

Note that this license only covers this Rust crate. The underlying Pylon library
has different license terms.
