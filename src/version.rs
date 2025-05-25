use crate::consts::CHAR_TABLE_VERSION;

/// Returns the embedded char-table version used at build time.
pub fn get_char_table_version() -> &'static str {
    CHAR_TABLE_VERSION
}
