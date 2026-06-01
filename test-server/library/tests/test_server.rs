#[cfg(test)]
mod tests {
    use std::{path::PathBuf};

    use audio_library::sources::directory::{Directory, options::DirectoryOptions};
    use test_server_lib::{error::Error, local::LocalServer, traits::Server};

    #[test]
    fn test_local_server_load_tracks() -> Result<(), Error> {
        let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap()
            .join("audio-library")
            .join("tests")
            .join("data")
            .join("messy-library");

        let sources = vec![
            Directory {
                path: messy_library_path,
                options: DirectoryOptions::default()
            }
        ];

        let mut server = LocalServer::new(sources);

        server.load_tracks()?;

        let state = server.get_state();

        let track_found = state.library.tracks.iter()
            .any(|(_, track)| *track.metadata.title == "Stardust");

        assert_eq!(track_found, true);

        Ok(())
    }
}