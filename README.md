This uses nightly Rust and https://github.com/japaric/xargo.

To build stuff you need:

    rustup install nightly
    rustup default nightly
    rustup component add rust-src
    cargo install xargo

And a cross-compiler for the OpenMono, on Ubuntu that is in
`gcc-arm-none-eabi`.

And finally the OpenMono framework source should be checked out next to the
checkout of this, and a `make release` run there.

After that the Makefile should work.

Note: Bit-rot seems to have set in, and this no longer works from a fresh
checkout. Will have to investigate.
