#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use audio_library::library::{Library, source::{LibraryLocalSource, LibrarySource}};

    #[test]
    fn test_library() {
        let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests").join("data").join("messy-library");

        assert_eq!(messy_library_path.exists(), true);

        let library = Library {
            source: LibrarySource::Local(
                LibraryLocalSource::Directory(messy_library_path)
            )
        };

        // TODO: test library parsing and check for correct track metadata
    }
}