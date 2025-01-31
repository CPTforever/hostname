pub fn get() -> io::Result<OsString> {
    return Err(Error::new(
        ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}

#[cfg(feature = "set")]
pub fn set(hostname: &OsStr) -> io::Result<()> {
    return Err(Error::new(
        ErrorKind::Unsupported,
        "Tell Ashley to complete this",
    )); 
}
