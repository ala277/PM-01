#[derive(Debug, Clone)]
pub struct PossibilitySpace {
    pub labels: Vec<String>,
}

impl PossibilitySpace {
    pub fn new(labels: Vec<String>) -> Result<Self, String> {
        if labels.is_empty() {
            return Err("PossibilitySpace cannot be empty".into());
        }

        Ok(Self { labels })
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }
}
