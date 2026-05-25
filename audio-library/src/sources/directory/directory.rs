use std::path::PathBuf;

use walkdir::WalkDir;

use crate::sources::{directory::track_iterator::DirectoryTrackIterator, traits::{Source, SourceOptions, SourceTrack}};

pub struct Directory {
    pub path: PathBuf,
    pub options: DirectoryOptions,
}

#[derive(Default, Clone)]
pub struct DirectoryOptions {
    pub trust_file_extension: bool
}

impl SourceOptions for DirectoryOptions {}

impl Source for Directory {}

impl SourceTrack for Directory {
    // TODO: switch to 'pub(crate)' when main.rs no longer uses this.
    /// Get and construct tracks from a local directory on disk.
    #[allow(refining_impl_trait)]
    fn get_tracks(&self) -> DirectoryTrackIterator {
        DirectoryTrackIterator {
            walk_dir_iterator: WalkDir::new(&self.path).into_iter(),
            options: self.options.clone()
        }
    }
}
