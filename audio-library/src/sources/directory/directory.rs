use std::path::PathBuf;

use walkdir::WalkDir;

use crate::sources::{directory::{options::DirectoryOptions, track_iterator::DirectoryTrackIterator}, traits::Source, r#type::SourceType};

pub struct Directory {
    pub path: PathBuf,
    pub options: DirectoryOptions,
}

impl Source for Directory {
    fn get_type(&self) -> SourceType {
        SourceType::Directory { path: self.path.clone() }
    }

    /// Get and construct tracks from a local directory on disk.
    #[allow(refining_impl_trait)]
    fn get_tracks(&self) -> DirectoryTrackIterator {
        DirectoryTrackIterator {
            walk_dir_iterator: WalkDir::new(&self.path).into_iter(),
            options: self.options.clone()
        }
    }
    
    // this probably will change
    // fn get_metadata(&self) -> SourceMetadata {
    //     SourceMetadata {
    //         location: format!("{}", self.path.display()),
    //         friendly_name: None
    //     }
    // }
}