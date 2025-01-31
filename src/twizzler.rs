pub fn get() -> io::Result<OsString> {
    return Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}

#[cfg(feature = "set")]
pub fn set(hostname: &OsStr) -> io::Result<()> {
    return Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}
