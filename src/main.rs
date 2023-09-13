// just a quick and dirty bitdiff implementation
use bitvec::prelude::*;
use std::{env, fs, path::PathBuf, process};

fn get_files(mut args: impl Iterator<Item = String>) -> Result<(PathBuf, PathBuf), &'static str> {
    args.next(); // ignore the program name

    let file_1 = match args.next() {
        Some(arg) => fs::canonicalize(arg),
        None => return Err("Didn't get input file 1!"),
    };

    let file_2 = match args.next() {
        Some(arg) => fs::canonicalize(arg),
        None => return Err("Didn't get input file 2!"),
    };

    if file_1.is_err() {
        return Err("Invalid argument passed for file_1");
    }
    let file_1 = file_1.unwrap();

    if file_2.is_err() {
        return Err("Invalid argument passed for file_2");
    }
    let file_2 = file_2.unwrap();

    if !file_1.is_file() {
        return Err("Invalid argument passed for file_1");
    }

    if !file_2.is_file() {
        return Err("Invalid argument passed for file_2");
    }

    return Ok((file_1, file_2));
}

fn cmp_files(buff_1: &Vec<u8>, buff_2: &Vec<u8>) -> usize {
    let n_diffs = buff_1.iter().zip(buff_2.iter()).fold(0, |accum, (a, b)| {
        let bits_1 = a.view_bits::<Msb0>();
        let bits_2 = b.view_bits::<Msb0>();

        accum
            + bits_1
                .iter()
                .zip(bits_2.iter())
                .fold(0, |mini_accum, (bit_a, bit_b)| {
                    mini_accum + if bit_a == bit_b { 0 } else { 1 }
                })
    });

    return n_diffs + (buff_1.len().abs_diff(buff_2.len()) * u8::BITS as usize);
}

fn main() {
    let (file_1, file_2) = get_files(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let buff_1 = std::fs::read(file_1).unwrap_or_else(|err| {
        eprintln!("Error occurred while opening file 1: {err}");
        process::exit(1);
    });

    let buff_2 = std::fs::read(file_2).unwrap_or_else(|err| {
        eprintln!("Error occurred while opening file 1: {err}");
        process::exit(1);
    });

    let diff = cmp_files(&buff_1, &buff_2);

    println!("Bitdiff: {diff}");
}
