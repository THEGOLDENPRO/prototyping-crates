use std::{fs::File, io::{BufReader, Read, Seek}, path::PathBuf};

use infer::MatcherType;
use walkdir::{WalkDir, IntoIter as WalkDirIterator};
use lofty::{aac::AacFile, ape::ApeFile, config::ParseOptions, error::{ErrorKind, LoftyError}, file::{AudioFile, FileType, TaggedFile, TaggedFileExt}, flac::FlacFile, iff::{aiff::AiffFile, wav::WavFile}, mp4::Mp4File, mpeg::MpegFile, musepack::MpcFile, ogg::{OpusFile, SpeexFile, VorbisFile}, tag::Accessor, wavpack::WavPackFile};

use crate::{error::{Error, Result}, inference::{AudioTitleParseRules, infer_and_parse_audio_title_from_path}, sources::{Source, SourceTrackResult}, track::{Track, metadata::{Metadata, MetadataField}, reader::TrackReader}};

pub struct Directory {
    pub path: PathBuf
}

impl Source for Directory {
    type TrackIterator = DirectoryTrackIterator;

    // NOTE: for now a vector for testing but 
    // this will very likely change to an iterator of some sort.

    // TODO: switch to 'pub(crate)' when main.rs no longer uses this.
    /// Get and construct tracks from a local directory on disk.
    fn get_tracks(&self) -> Self::TrackIterator {
        DirectoryTrackIterator {
            walk_dir_iterator: WalkDir::new(&self.path).into_iter(),
        }
    }
}

// TODO: make pub(crate)
pub struct DirectoryTrackIterator {
    walk_dir_iterator: WalkDirIterator,
}

// TODO: make TrackIterator trait that implements Iterator and yields SourceTrackResult

impl Iterator for DirectoryTrackIterator {
    type Item = SourceTrackResult;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(library_entry) = self.walk_dir_iterator.next() {
            let library_entry = match library_entry {
                Ok(dir_entry) => dir_entry,
                Err(error) => {
                    return Some(
                        SourceTrackResult::Error(
                            Error::RecursiveWalkFailure { error: error.to_string() }
                        )
                    );
                },
            };

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

            let audio_file = match File::open(library_path) {
                Ok(file) => file,
                Err(error) => {
                    return Some(
                        SourceTrackResult::Error(
                            Error::FileOpenFailure {
                                file_name: audio_file_name,
                                error: error.to_string()
                            }
                        )
                    );
                },
            };
            let mut audio_file_buf_reader = BufReader::new(
                TrackReader::File(audio_file)
            );

            let mut audio_file_header_buffer = [0; 64];
            if let Err(error) = audio_file_buf_reader.read_exact(&mut audio_file_header_buffer) {
                return Some(
                    SourceTrackResult::Error(
                        Error::FileFormatReadHeaderFailure {
                            file_name: audio_file_name,
                            error: error.to_string()
                        }
                    )
                );
            }

            log::debug!("Detecting '{audio_file_name}' file format with infer...");

            let detected_audio_format = match infer::get(&audio_file_header_buffer) {
                Some(file_format) => {
                    match file_format.matcher_type() {
                        MatcherType::Audio => file_format,
                        MatcherType::Video => file_format,
                        other_format_type => {
                            log::debug!(
                                "Skipping the '{}' file as it is not an \
                                audio file (got '{other_format_type:?}' instead)...",
                                library_entry.file_name().display(),
                            );

                            continue;
                        }
                    }
                },
                None => {
                    return Some(
                        SourceTrackResult::Error(
                            Error::UnknownFileFormat { file_name: audio_file_name }
                        )
                    );
                },
            };

            let detected_file_extension = detected_audio_format.extension();

            log::debug!("Parsing the detected file extension '{detected_file_extension}' to a lofty file type...");

            let lofty_file_type = match FileType::from_ext(detected_file_extension) {
                Some(file_type) => file_type,
                None => {
                    return Some(
                        SourceTrackResult::Error(
                            Error::AudioFormatExtensionParseFailure {
                                audio_file_name,
                                format: detected_file_extension.to_string()
                            }
                        )
                    );
                },
            };

            let source_track_result = match read_and_parse_audio_file_header(&mut audio_file_buf_reader, lofty_file_type) {
                Ok(tagged_file) => {
                    if let Some(tag) = tagged_file.primary_tag() {
                        if let Some(title_string) = tag.title() {
                            metadata.title = MetadataField {
                                value: title_string.to_string(),
                                inferred: false
                            };
                        }
                    };

                    SourceTrackResult::Track(
                        Track { metadata, buf_reader: audio_file_buf_reader }
                    )
                },
                Err(error) => {
                    // TODO: failed metadata extraction list maybe??
                    SourceTrackResult::PartialTrack(
                        Track { metadata, buf_reader: audio_file_buf_reader },
                        vec![
                            Error::AudioTagsParseFailure {
                                audio_file_name,
                                error: error.to_string()
                            }
                        ]
                    )
                },
            };

            return Some(source_track_result);
        }

        None
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