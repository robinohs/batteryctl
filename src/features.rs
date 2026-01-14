#[derive(Debug)]
pub struct Features {
    pub has_carbond: bool,
}

impl Features {
    /// Detects the features available in the current environment.
    pub fn detect() -> eyre::Result<Features> {
        Ok(Features {
            has_carbond: which::which("carbond").is_ok(),
        })
    }
}
