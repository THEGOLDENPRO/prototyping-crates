use std::path::PathBuf;

use walkdir::WalkDir;

use crate::sources::{Source, SourceOptions, directory::track_iterator::DirectoryTrackIterator};

pub struct Directory {
    pub path: PathBuf,
    pub options: DirectoryOptions,
}

#[derive(Default, Clone)]
pub struct DirectoryOptions {
    pub trust_file_extension: bool
}

impl SourceOptions for DirectoryOptions {}

impl Source for Directory {
    // NOTE: for now a vector for testing but 
    // this will very likely change to an iterator of some sort.

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
