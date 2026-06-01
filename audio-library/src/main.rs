use log::LevelFilter;
use std::{env, path::PathBuf};
use audio_library::sources::{directory::{Directory, options::DirectoryOptions}, traits::Source};

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let path_string: String = env::args().skip(1).collect();

    let library_path = match path_string.is_empty() {
        true => PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join("messy-library"),
        false => PathBuf::from(path_string),
    };

    log::info!("Reading library at '{}'...", library_path.display());

    let directory = Directory {
        path: library_path,
        options: DirectoryOptions::default()
    };

    for track_result in directory.get_tracks() {
        match track_result {
            Ok(track) => {
                println!("Track Name: {}", *track.metadata.title);
                println!("Track Issues: {:?}", *&track.issues);
            },
            Err(error) => {
                log::error!("Failed Track Error: {:?}", error);
            },
        }
    }
}