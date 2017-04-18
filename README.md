This uses nightly Rust and https://github.com/japaric/xargo.

To build stuff you need:

  rustup install nightly
  rustup default nightly
  rustup component add rust-src
  cargo install xargo

After that the Makefile should work.
