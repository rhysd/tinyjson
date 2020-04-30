#![no_main]
use libfuzzer_sys::fuzz_target;

use std::io::{self, Write};
use std::str;
use tinyjson::JsonParseResult;

fn print_bytes<W: Write>(mut w: W, b: &[u8]) {
    w.write_all(b"let input = b\"").unwrap();
    for b in b.iter() {
        write!(w, "\\x{:02x}", b).unwrap();
    }
    w.write_all(b"\"\n").unwrap();
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        // Printing input makes fuzzer slow. But it is necessary because cargo-fuzz does not output
        // report on abort. To know what the input was, it is necessary to output the input before
        // running a parser.
        print_bytes(io::stderr().lock(), s.as_bytes());
        let _: JsonParseResult = s.parse();
    }
});
