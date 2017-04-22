# Using `data-encoding` for MIME Encoding

[`rustc-serialize`](https://github.com/rust-lang-deprecated/rustc-serialize) has been deprecated, with its
(de)serialization being deprecated in favour of [`serde`](https://serde.rs/).

Its base64 and hex features, however, are replaced by other crates such as
[`data-encoding`](https://github.com/ia0/data-encoding).

The only thing that `data-encoding` is missing is out of the box support for
["MIME"](https://docs.rs/rustc-serialize/0.3.24/rustc_serialize/base64/static.MIME.html) encoding. But "MIME" encoding
is basically just base64 padded encoding with line breaks every 76 characters.

This repository is just a simple implementation to show how to acheive the same thing with `data-encoding` easily.

## Usage
Demonstration can be run with `cargo run`. Otherwise, the source code should suffice for the purpose of demonstration.
