pub const XZ_EXTS: &[&str] = &["xz"];
pub const ZST_EXTS: &[&str] = &["zstd", "zst"];
pub const GZ_EXTS: &[&str] = &["gz", "gzip"];
pub const XZ_MIME: &str = "application/x-xz";
pub const ZST_MIME: &str = "application/zstd";
pub const GZ_MIME: &str = "application/gzip";
pub const BZ2_MIME: &str = "application/x-bzip2";
pub const SUPPORTED_MIME_TYPES: &[&str] = &[XZ_MIME, ZST_MIME, GZ_MIME, BZ2_MIME];
