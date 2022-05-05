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


//pub trait MaybeError<T> {
//    fn or_exit(self, message: &str) -> T;
//}
//
//impl<T, E> MaybeError<T> for Result<T, E>
//where
//    E: std::fmt::Display + std::fmt::Debug,
//{
//    fn or_exit(self, message: &str) -> T {
//        if let Err(e) = self {
//            eprintln!("Failed to {}: {}", message, e);
//        }
//
//        self.unwrap()
//    }
//}
//
//impl<T> MaybeError<T> for Option<T> {
//    fn or_exit(self, message: &str) -> T {
//        if self.is_none() {
//            eprintln!("Failed to {}", message);
//        }
//
//        self.unwrap()
//    }
//}

fuzz_target!(|data: &[u8]| {
    let temp_directory = env::temp_dir();
    let filename = temp_directory.join("file").into_os_string().into_string().unwrap();
    let mut file = File::create(filename).unwrap();
    file.write(data).unwrap();
    let filename = temp_directory.join("file").into_os_string().into_string().unwrap();

    let contents = std::fs::read(&filename);
    let contents = match contents {
        Ok(c) => c,
        Err(error) => {
            format!("Couldn't read file \"{}\"", filename);
            return ();
        },
    };
    let elf = ParsedElf::from_bytes(&filename, &contents);
    let elf = match elf {
        Ok(e) => e,
        Err(error) => {
            format!("Couldn't parse file \"{}\"", filename);
            return ();
        },
    };
    let report_filename = utils::construct_filename(&filename);
    let report_filename = match report_filename {
        Some(c) => c,
        None => {
            format!("Couldn't construct file \"{}\"", filename);
            return ();
        },
    };
    let report = report_gen::generate_report(&elf);
    let report = match report {
        Some(c) => c,
        None => {
            format!("Couldn't generate report");
            return ();
        },
    };

    std::fs::write(report_filename, report);
});
