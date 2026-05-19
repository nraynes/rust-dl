use rust_alert::alert;

/// A custom error type used to convert error types from various crates.
#[alert(errors = [reqwest::Error, std::io::Error])]
pub struct DownloaderError {}
