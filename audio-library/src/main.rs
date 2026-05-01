use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use infer::MatcherType;
use lofty::{file::FileType, probe::Probe as LoftyProbe};
use log::{LevelFilter, debug, error};
use walkdir::WalkDir;

type Probe<'a> = LoftyProbe<&'a mut BufReader<File>>;

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    const STRICT_MODE: bool = true;

    const USE_FILE_EXTENSION: bool = STRICT_MODE;

    let messy_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("messy-library");

    for library_entry in WalkDir::new(messy_library_path) {
        let library_entry = library_entry.unwrap();
        let library_path = library_entry.path();

        if library_path.is_dir() {
            continue;
        }

        let audio_file_name = library_entry.file_name().display();

        let audio_file = File::open(library_path).unwrap();
        let mut audio_file_buf_reader = BufReader::new(audio_file);

        let mut audio_file_header_buffer = [0; 64];
        if let Err(error) = audio_file_buf_reader.read_exact(&mut audio_file_header_buffer) {
            // TODO: add to list of failed files

            log::error!(
                "Failed to read the file ('{}') header to get audio format! Error: {error}",
                audio_file_name
            );
        }

        debug!("Detecting '{audio_file_name}' file format with infer...");

        let detected_audio_format = match infer::get(&audio_file_header_buffer) {
            Some(file_format) => {
                match file_format.matcher_type() {
                    MatcherType::Audio => file_format,
                    MatcherType::Video => file_format,
                    other_format_type => {
                        debug!(
                            "Skipping the '{}' file as it is not an \
                            audio file (got '{other_format_type:?}' instead)...",
                            library_entry.file_name().display(),
                        );

                        continue;
                    }
                }
            },
            None => {
                debug!(
                    "Skipping the '{audio_file_name}' file as we cannot detect this file's format..."
                );

                continue;
            },
        };

        let detected_file_extension = detected_audio_format.extension();

        debug!("Parsing the detected file extension '{detected_file_extension}' to a lofty file type...");

        let lofty_file_type = match FileType::from_ext(detected_file_extension) {
            Some(file_type) => file_type,
            None => {
                // TODO: add to list of failed files

                error!(
                    "Lofty could not parse audio file format \
                    extension ('{detected_file_extension}') for '{audio_file_name}'!"
                );

                continue;
            },
        };

        let audio_probe = Probe::new(&mut audio_file_buf_reader)
            .set_file_type(lofty_file_type);

        println!("{:?}", audio_probe.file_type());
    }
}