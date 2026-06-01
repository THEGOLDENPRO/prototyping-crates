use std::fmt::Display;

#[derive(Debug)]
pub enum TrackIssues {
    TagsParseFailure { error: String }
}

impl Display for TrackIssues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackIssues::TagsParseFailure { error } => write!(
                f,
                "Failed to extract tags! This file will lack metadata! Error: {error}"
            ),
        }
    }
}