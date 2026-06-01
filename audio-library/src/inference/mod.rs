use std::path::Path;

use lofty::file::EXTENSIONS;

#[derive(Default)]
pub struct AudioTitleParseRules {
    keep_track_numbers: bool,
    keep_square_brackets: bool,
}

// TODO: infer track number

pub fn infer_and_parse_audio_title(file_name: &String, path: &Path, parsing_rules: AudioTitleParseRules) -> String {
    let mut formatted_title = file_name.replace("_", " ");

    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            if EXTENSIONS.contains(&extension_str) {
                formatted_title = formatted_title.replace(format!(".{extension_str}").as_str(), "");
            }
        }
    }

    if !parsing_rules.keep_square_brackets {
        formatted_title.retain(|c| c != '[' && c != ']');
    }

    if !parsing_rules.keep_track_numbers {
        formatted_title = formatted_title
            .trim_start_matches(|c: char| c.is_ascii_digit())
            .trim_start_matches('.')
            .trim_start()
            .to_string();
    }

    formatted_title
}