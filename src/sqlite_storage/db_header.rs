use crate::bytes::from_be_bytes::from_be_bytes;

#[derive(Debug)]
pub struct DBHeader {
    // The header string: "SQLite format 3\000"
    pub page_size: u16,
    // File format write version
    // File format read version
    pub reserved_size: u8,
    // Maximum embedded payload fraction
    // Minimum embedded payload fraction
    // Leaf payload fraction
    // File change counter
    pub page_cnt: u32,
    // Page number of the first freelist trunk page
    // Total number of freelist pages
    // The schema cookie
    // The schema format number. Supported schema formats are 1, 2, 3, and 4
    // Default page cache size
    // The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise
    pub text_encoding: u32,
    // The "user version" as read and set by the user_version pragma
    // True (non-zero) for incremental-vacuum mode. False (zero) otherwise
    // The "Application ID" set by PRAGMA application_id
    // Reserved for expansion. Must be zero
    // The version-valid-for number
    // SQLITE_VERSION_NUMBER
}

impl DBHeader {
    pub fn parse(bytes: [u8; 100]) -> Self {
        Self {
            page_size: from_be_bytes(&mut &bytes[16..]),
            reserved_size: from_be_bytes(&mut &bytes[20..]),
            page_cnt: from_be_bytes(&mut &bytes[28..]),
            text_encoding: from_be_bytes(&mut &bytes[56..]),
        }
    }
}
