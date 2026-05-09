use std::{fs::File, io::{BufReader, Read, Seek}, path::PathBuf};

use audio_library::{error::Result, inference::{AudioTitleParseRules, infer_and_parse_audio_title_from_path}, track::{Track, metadata::{Metadata, MetadataField}}};
use infer::MatcherType;
use lofty::{aac::AacFile, ape::ApeFile, config::ParseOptions, error::{ErrorKind, LoftyError}, file::{AudioFile, FileType, TaggedFile, TaggedFileExt}, flac::FlacFile, iff::{aiff::AiffFile, wav::WavFile}, mp4::Mp4File, mpeg::MpegFile, musepack::MpcFile, ogg::{OpusFile, SpeexFile, VorbisFile}, probe::Probe as LoftyProbe, tag::Accessor, wavpack::WavPackFile};
use log::{LevelFilter, debug, error};
use walkdir::WalkDir;

pub type Probe<'a> = LoftyProbe<BufReader<File>>;

// TODO: move main.rs logic to directory source

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

        let audio_file_name = library_entry.file_name().to_string_lossy().to_string();

        let inferred_title = infer_and_parse_audio_title_from_path(
            &library_path,
            &audio_file_name,
            AudioTitleParseRules::default()
        );

        let mut metadata = Metadata {
            title: MetadataField { value: inferred_title, inferred: true}
        };

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

        match read_and_parse_audio_file_header(&mut audio_file_buf_reader, lofty_file_type) {
            Ok(tagged_file) => {
                if let Some(tag) = tagged_file.primary_tag() {
                    if let Some(title_string) = tag.title() {
                        metadata.title = MetadataField {
                            value: title_string.to_string(),
                            inferred: false
                        };
                    }
                };
            },
            Err(error) => {
                // TODO: failed metadata extraction list maybe??
                log::warn!(
                    "Failed to extract tags from the audio file '{audio_file_name}'! \
                        This file will lack metadata! Error: {error}"
                );
            },
        }

        let track = Track { metadata, buf_reader: audio_file_buf_reader };

        println!("Name: {:?}", *track.metadata.title);
    }
}

fn read_and_parse_audio_file_header<R: Read + Seek>(audio_file_buf_reader: &mut BufReader<R>, file_type: FileType) -> Result<TaggedFile, LoftyError> {
    let parse_options = ParseOptions::default();

    let tagged_file: TaggedFile = match file_type {
        FileType::Aac => AacFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Aiff => AiffFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Ape => ApeFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Flac => FlacFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Mpeg => MpegFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Opus => OpusFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Vorbis => VorbisFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Wav => WavFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Mp4 => Mp4File::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Mpc => MpcFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::Speex => SpeexFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        FileType::WavPack => WavPackFile::read_from(audio_file_buf_reader, parse_options)?.into(),
        unsupported_type => return Err(LoftyError::new(ErrorKind::UnknownFormat)),
    };

    Ok(tagged_file)
}