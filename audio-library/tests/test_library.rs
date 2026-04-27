#[cfg(test)]
mod tests {
    use core::panic;
    use std::path::PathBuf;

    use audio_library::{library::Library, source::{LocalSource, Source}};

    #[test]
    fn test_library() {
        let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests").join("data").join("messy-library");

        assert_eq!(messy_library_path.exists(), true);

        let library = Library {
            source: Source::Local(
                LocalSource::Directory(messy_library_path)
            )
        };

        // TODO: test library parsing and check for correct track metadata
    }
}