#![no_main]

use libfuzzer_sys::fuzz_target;

#[path = "../../src/elf/mod.rs"]
mod elf;
#[path = "../../src/report_gen.rs"]
mod report_gen;
#[path = "../../src/utils.rs"]
mod utils;

use std::env;
use std::fs::File;
use std::io::Write;
use elf::parser::ParsedElf;

fuzz_target!(|data: &[u8]| {
    let temp_directory = env::temp_dir();
    let filename = temp_directory.join("file").into_os_string().into_string().unwrap();
    let mut file = File::create(filename).unwrap();
    file.write(data).unwrap();
    let filename = temp_directory.join("file").into_os_string().into_string().unwrap();

    let contents = std::fs::read(&filename);
    let contents = match contents {
        Ok(c) => c,
        Err(_) => {
            return ();
        },
    };
    let elf = ParsedElf::from_bytes(&filename, &contents);
    let elf = match elf {
        Ok(e) => e,
        Err(_) => {
            return ();
        },
    };
    let report_filename = utils::construct_filename(&filename);
    let report_filename = match report_filename {
        Some(c) => c,
        None => {
            return ();
        },
    };
    let report = report_gen::generate_report(&elf);

    std::fs::write(report_filename, report);
});
