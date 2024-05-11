use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::num::ParseIntError;

// Decode 2 bytes per target byte, expecting ASCII encoded hex representation.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn hex_file_to_bin(filename: &str) -> Vec<u8> {
    let mut result = Vec::new();

    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        let len = line.len();
        if len % 2 != 0 {
            panic!("Line {i}: uneven number of characters ({len})")
        }
        match decode_hex(line) {
            Ok(l) => {
                for b in l {
                    result.push(b)
                }
            }
            Err(e) => panic!("Line {i}: {e}"),
        }
    }
    result
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no file given",
        ));
    }
    let hexfile = &args[1];
    let binfile = format!("{hexfile}.bin");
    let res = hex_file_to_bin(hexfile);
    println!("Writing binary to\n  \"{binfile}\"");
    File::create(binfile)?.write_all(&res)?;
    Ok(())
}
