pub struct SourceMetadata {
    // should location be Option<String>? can it ever be unknown??
    pub location: String,
    pub friendly_name: Option<String>,
}