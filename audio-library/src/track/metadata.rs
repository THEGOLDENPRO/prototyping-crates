use std::ops::Deref;

pub struct Metadata {
    pub title: MetadataField<String>,
}

impl Metadata {}

pub struct MetadataField<T> {
    pub value: T,
    pub inferred: bool,
}

// umm, is it a good idea to implement deref?
impl<T> Deref for MetadataField<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// pub enum MetadataSource {
//     FilePath,
//     AudioTags,
//     Jellyfin,
// }

// impl Display for MetadataSource {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             MetadataSource::FilePath => write!(f, "File Path"),
//             MetadataSource::AudioTags => write!(f, "Audio Tags (e.g: Id3v2 tags, Vorbis comments, etc)"),
//             MetadataSource::Jellyfin => write!(f, "Jellyfin"),
//         }
//     }
// }