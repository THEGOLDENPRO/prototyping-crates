use log::LevelFilter;
use std::path::PathBuf;
use audio_library::sources::{Source, SourceTrackResult, directory::Directory};

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    const STRICT_MODE: bool = true;

    const USE_FILE_EXTENSION: bool = STRICT_MODE;

    let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("messy-library");

    let directory = Directory { path: messy_library_path };

    for track_result in directory.get_tracks() {
        match track_result {
            SourceTrackResult::Track(track) => {
                println!("Track Name: {}", *track.metadata.title);
            },
            SourceTrackResult::PartialTrack(track, errors) => {
                println!("Partial Track Name: {}", *track.metadata.title);
                println!("Partial Track Errors: {:?}", errors);
            },
            SourceTrackResult::Error(error) => {
                println!("Failed Track Error: {:?}", error);
            },
        }
    }
}