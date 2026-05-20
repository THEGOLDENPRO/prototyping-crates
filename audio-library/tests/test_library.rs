#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use audio_library::{library::{Library, source::LibrarySource}, sources::directory::{Directory, DirectoryOptions}};

    #[test]
    fn test_library() {
        let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests").join("data").join("messy-library");

        assert_eq!(messy_library_path.exists(), true);

        let directory = Directory {
            path: messy_library_path,
            options: DirectoryOptions::default()
        };

        let mut library = Library::new();

        library.add_source(LibrarySource::Directory(directory));

        // TODO: test library parsing and check for correct track metadata
    }
}