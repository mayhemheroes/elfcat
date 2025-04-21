use std::path::Path;

pub fn human_format_bytes(bytes: u64) -> String {
    let base: u64 = 1024;

    if bytes < base {
        return format!("{} B", bytes);
    }

    let prefixes = ["K", "M", "G", "T", "P", "E"];
    let exponent = (bytes as f64).log(base as f64) as u32;

    format!(
        "{:.1} {}iB",
        (bytes as f64) / (base.pow(exponent) as f64),
        prefixes[(exponent - 1) as usize]
    )
}

pub fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&apos;"),
        _ => None,
    }
}

pub trait MaybeError<T> {
    fn or_exit(self, message: &str) -> T;
}

impl<T, E> MaybeError<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn or_exit(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to {}: {}", message, e);
                std::process::exit(1);
            }
        }
    }
}

impl<T> MaybeError<T> for Option<T> {
    fn or_exit(self, message: &str) -> T {
        if self.is_none() {
            eprintln!("Failed to {}", message);
            std::process::exit(1);
        }

        self.unwrap()
    }
}

pub fn basename(path: &str) -> Option<&str> {
    Path::new(path).file_name()?.to_str()
}

pub fn construct_filename(filename: &str) -> Option<String> {
    let name = basename(filename)?.to_string() + ".html";

    Some(name)
}
