extern crate data_encoding;

use std::cmp::min;
use std::str;

// According to https://docs.rs/rustc-serialize/0.3.24/rustc_serialize/base64/static.MIME.html
static LINE_ENDINGS: &'static str = "\r\n";
const LINE_LENGTH: usize = 76;
const ENCODING: &'static data_encoding::Padded = data_encoding::BASE64;

/// "MIME" encoding -- Use padded Base64 encoding, and add CRLF every 76 chracters, except for the
/// end
fn mime_encode(input: &[u8]) -> String {
    let base64 = ENCODING.encode(input);
    let chunks = split(&base64, LINE_LENGTH);
    chunks.join(LINE_ENDINGS).to_string()
}

// "MIME" decoding -- Strip "\r\n" and then decode as usual
fn mime_decode(input: &str) -> Result<Vec<u8>, data_encoding::DecodeError> {
    let stripped = input.replace("\r\n", "");
    ENCODING.decode(stripped.as_bytes())
}

/// Splits a given &str into chunks of appropriate length
/// Note: This assumes each character is one byte -- because we are dealing with output from Base64,
/// this should be OK. Otherwise, this might panic.
/// See the section `impl Index<Range<usize>> for str` on
/// https://doc.rust-lang.org/std/primitive.str.html
fn split<'a>(s: &'a str, len: usize) -> Vec<&'a str> {
    let num_chunks = s.len() / len;
    let num_chunks = if s.len() % len > 0 {
        num_chunks + 1
    } else {
        num_chunks
    };

    (0..num_chunks)
        .map(|i| {
                 let start = i * len;
                 let end = min(start + len, s.len());
                 &s[start..end]
             })
        .collect()
}

fn main() {
    let example = "PGh0bWw+CiAgPGhlYWQ+CiAgPC9oZWFkPgogIDxib2R5PgogICAgPHA+VGhpcyBpcyB0aGUgYm9k\r\neSBvZiB0aGUgbWVzc2FnZS48L3A+CiAgPC9ib2R5Pgo8L2h0bWw+Cg==";

    let decoded = mime_decode(&example).unwrap();
    let s = str::from_utf8(&decoded).unwrap();
    println!("{}", s);

    let encoded = mime_encode(s.as_bytes());

    assert_eq!(example, encoded);
}
