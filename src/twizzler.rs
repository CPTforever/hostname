use std::ffi::OsString;

pub fn get() -> std::io::Result<OsString> {
    return Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}

#[cfg(feature = "set")]
pub fn set(hostname: &OsStr) -> std::io::Result<()> {
    return Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}
