use crate::sources::traits::SourceOptions;

#[derive(Default, Clone)]
pub struct DirectoryOptions {
    pub trust_file_extension: bool,
    pub skip_parsing_track_tags: bool,
}

impl SourceOptions for DirectoryOptions {}