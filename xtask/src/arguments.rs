use std::path::PathBuf;

pub(crate) struct Arguments(std::vec::IntoIter<String>);

impl Arguments {
    pub fn from_environment() -> Self {
        Self::new(std::env::args().skip(1).collect())
    }
    pub fn new(values: Vec<String>) -> Self {
        Self(values.into_iter())
    }
    pub fn value(&mut self, option: &str) -> Result<String, String> {
        self.0
            .next()
            .ok_or_else(|| format!("{option} requires a value"))
    }
    pub fn path(&mut self, option: &str) -> Result<PathBuf, String> {
        self.value(option).map(PathBuf::from)
    }
    pub fn number<T: std::str::FromStr>(&mut self, option: &str) -> Result<T, String> {
        self.value(option)?
            .parse()
            .map_err(|_| format!("{option} requires a number"))
    }
}

impl Iterator for Arguments {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.0.next()
    }
}

pub(crate) fn unknown(argument: &str) -> String {
    format!("unknown argument: {argument}; use --help")
}
