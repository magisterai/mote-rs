#[path = "support/line_capture.rs"]
mod line_capture;
use std::{error::Error, fs, path::Path};
fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let fixture = arguments.next().ok_or("expected fixture directory")?;
    let output = arguments.next().ok_or("expected output JSON path")?;
    if arguments.next().is_some() {
        return Err("unexpected argument".into());
    }
    let root = Path::new(&fixture);
    let requests = serde_json::from_slice(&fs::read(root.join("requests.json"))?)?;
    fs::write(
        output,
        serde_json::to_string_pretty(&line_capture::measure_cases(root, &requests)?)? + "\n",
    )?;
    Ok(())
}
