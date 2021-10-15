pub struct Source {
    pub url: String,
    pub contense: String,
}

impl Default for Source {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            contense: "".to_owned(),
        }
    }
}

/// Processes a source. Returns the contense of the post.
pub fn process_source(url: String) -> Source {
    Source::default()
}

