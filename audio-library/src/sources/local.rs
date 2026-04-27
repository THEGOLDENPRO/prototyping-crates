
// umm thinking about how I would go about this...
// might just detect all structure types but only to get missing metadata (album, author)

pub enum LocalLibraryStructure {
    /// Structural Examples:
    /// - `Album Name / Audio Track Title`
    /// - `Artist Name / Audio Track Title`
    /// - `Artist Name / Album Name / Audio Track Title`
    /// 
    /// Actual Examples:
    /// - `~/Music/Artist/Album (year)/Track Title.mp3`
    /// - `~/Music/Artist/Album (year)/01. Track Title.mp3`
    /// - `~/Music/Some Artist/Album 1/Song 1.flac`
    /// - `~/Music/Album 1/Disk 1 Track 1.ogg`
    /// - `~/Music/Album 2/Disk 1/Disc 1 Track 1.aiff`
    /// - `~/Music/Queen - Bohemian Rhapsody.mp3`
    /// - `~/Music/Lamp/Lamp - Yume (2014)/01. シンフォニー.flac`
    /// - `~/Music/yes mama, OK?/Century End_s Order/06. steeplechase.flac`
    HierarchyStandard,
    /// There is no library structure or it's the flat structure.
    /// 
    /// Structural Examples:
    /// - `Audio Track Title`
    /// 
    /// Actual Examples:
    /// - `~/Music/track.mp3`
    /// - `~/Music/yes mama, OK? - steeplechase.flac`
    /// - `~/Music/Queen - Bohemian Rhapsody.mp3`
    None,
}