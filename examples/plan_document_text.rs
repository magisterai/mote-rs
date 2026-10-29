#[path = "support/document_capture.rs"]
mod document_capture;
use std::{env, error::Error, fs};
fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = env::args_os().skip(1).collect();
    if arguments.len() != 2 {
        return Err("usage: plan_document_text REQUESTS.json OUTPUT.json".into());
    }
    let requests = serde_json::from_slice(&fs::read(&arguments[0])?)?;
    let actual = document_capture::capture_sources(&requests)?;
    fs::write(&arguments[1], serde_json::to_vec_pretty(&actual)?)?;
    Ok(())
}
