/// Adapt a `std::io::Write` type into a `std::fmt::Write`.
pub struct IoAdapter<'a, T: std::io::Write> {
    inner: &'a mut T,
}

impl<'a, T: std::io::Write> IoAdapter<'a, T> {
    /// Create a new `IoAdapter` that wraps the given `std::io::Write` type.
    pub fn new(writer: &'a mut T) -> Self {
        IoAdapter { inner: writer }
    }
}

impl<T: std::io::Write> std::fmt::Write for IoAdapter<'_, T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
