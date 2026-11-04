//! Exports a DOCX or PPTX package to PDF through Microsoft Word or PowerPoint on
//! macOS, the reference renderers for Mote's native layout.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Application {
    Word,
    PowerPoint,
}

impl Application {
    fn name(self) -> &'static str {
        match self {
            Self::Word => "Microsoft Word",
            Self::PowerPoint => "Microsoft PowerPoint",
        }
    }

    /// The sandboxed applications may only open and save files inside their own
    /// containers without asking for permission.
    fn container(self) -> Result<PathBuf, String> {
        let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
        let identifier = match self {
            Self::Word => "com.microsoft.Word",
            Self::PowerPoint => "com.microsoft.Powerpoint",
        };
        Ok(Path::new(&home)
            .join("Library/Containers")
            .join(identifier)
            .join("Data/Documents/mote-office-compare"))
    }

    /// Word prints comment balloons in a markup pane that shrinks the page, so the
    /// script hides comments and revisions before exporting.
    fn script(self, package: &Path, pdf: &Path) -> String {
        let (package, pdf) = (package.display(), pdf.display());
        match self {
            Self::Word => format!(
                r#"tell application "Microsoft Word"
	set display alerts to alerts none
	open POSIX file "{package}"
	delay 1
	set exported to active document
	try
		set show revisions and comments of view of active window to false
	end try
	try
		set show comments of view of active window to false
	end try
	save as exported file name "{pdf}" file format format PDF
	close exported saving no
end tell"#
            ),
            Self::PowerPoint => format!(
                r#"tell application "Microsoft PowerPoint"
	open POSIX file "{package}"
	delay 1
	set exported to active presentation
	save exported in POSIX file "{pdf}" as save as PDF
	close exported saving no
end tell"#
            ),
        }
    }

    /// Returns the application's version, recorded beside each reference.
    pub fn version(self) -> Result<String, String> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "tell application \"{}\" to get version",
                self.name()
            ))
            .output()
            .map_err(|error| format!("osascript: {error}"))?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
        }
        Ok(format!(
            "{} {}",
            self.name(),
            String::from_utf8_lossy(&output.stdout).trim()
        ))
    }

    /// Opens the package in the application, saves it as PDF and returns the PDF.
    pub fn export(self, stem: &str, package: &[u8]) -> Result<Vec<u8>, String> {
        let container = self.container()?;
        fs::create_dir_all(&container)
            .map_err(|error| format!("{}: {error}", container.display()))?;
        let extension = match self {
            Self::Word => "docx",
            Self::PowerPoint => "pptx",
        };
        let package_path = container.join(format!("{stem}.{extension}"));
        let pdf_path = container.join(format!("{stem}.pdf"));
        fs::write(&package_path, package).map_err(|error| error.to_string())?;
        let _ = fs::remove_file(&pdf_path);
        let result = self.run(&package_path, &pdf_path);
        let _ = fs::remove_file(&package_path);
        let _ = fs::remove_file(&pdf_path);
        result
    }

    fn run(self, package: &Path, pdf: &Path) -> Result<Vec<u8>, String> {
        let mut child = Command::new("osascript")
            .arg("-e")
            .arg(self.script(package, pdf))
            .spawn()
            .map_err(|error| format!("osascript: {error}"))?;
        let start = Instant::now();
        loop {
            if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
                if !status.success() {
                    return Err(format!("{} could not export the package", self.name()));
                }
                break;
            }
            if start.elapsed() > Duration::from_secs(120) {
                let _ = child.kill();
                return Err(format!("{} did not finish the export", self.name()));
            }
            thread::sleep(Duration::from_millis(200));
        }
        fs::read(pdf).map_err(|error| format!("{} wrote no PDF: {error}", self.name()))
    }
}
