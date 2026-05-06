use std::path::Path;

use lofty::file::EXTENSIONS;

#[derive(Default)]
pub struct AudioTitleParseRules {
    keep_square_brackets: bool
}

pub fn infer_and_parse_audio_title_from_path(path: &Path, file_name: &String, parsing_rules: AudioTitleParseRules) -> String {
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

    formatted_title
}