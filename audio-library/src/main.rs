use log::LevelFilter;
use std::path::PathBuf;
use audio_library::sources::{directory::{Directory, DirectoryOptions}, result::SourceTrackResult, traits::SourceTrack};

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("messy-library");

    let directory = Directory {
        path: messy_library_path,
        options: DirectoryOptions::default()
    };

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