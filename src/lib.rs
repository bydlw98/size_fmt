#![no_std]
#![doc = include_str!("../README.md")]
mod size_fmt_impl;

#[cfg(test)]
mod tests;

pub use size_fmt_impl::Integer;

/// A correctly sized stack allocated array of bytes for the formatted size to be written into.
#[derive(Default)]
pub struct Buffer {
    inner: size_fmt_impl::Buffer,
}

impl Buffer {
    /// Creates a new [`Buffer`].
    pub fn new() -> Self {
        let inner = size_fmt_impl::Buffer::new();

        Self { inner }
    }

    /// Formats size into this buffer and returns a str reference to it.
    ///
    /// Size is formatted as its raw representation like 1000, 2000000, 3000000000 etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut buffer = size_fmt::Buffer::new();
    /// let printed = buffer.raw_fmt(4096);
    ///
    /// assert_eq!(printed, "4096");
    /// ```
    pub fn raw_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.raw_fmt(size)
    }

    /// Formats size into this buffer and returns a str reference to it.
    ///
    /// Size is formatted with factors of 1024 like 1.0K 200M 3.0G etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut buffer = size_fmt::Buffer::new();
    /// let printed = buffer.human_fmt(4096);
    ///
    /// assert_eq!(printed, "4.0K");
    /// ```
    pub fn human_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.human_fmt(size)
    }

    /// Formats size into this buffer and returns a str reference to it.
    ///
    /// Size is formatted with factors of 1000 like 1.0k 200M 3.0G etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut buffer = size_fmt::Buffer::new();
    /// let printed = buffer.si_fmt(4096);
    ///
    /// assert_eq!(printed, "4.1k");
    /// ```
    pub fn si_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.si_fmt(size)
    }

    /// Formats size into this buffer and returns a str reference to it.
    ///
    /// Size is formatted with factors of 1024 like 1.0Ki 200Mi 3.0Gi etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut buffer = size_fmt::Buffer::new();
    /// let printed = buffer.iec_fmt(4096);
    ///
    /// assert_eq!(printed, "4.0Ki");
    /// ```
    pub fn iec_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.iec_fmt(size)
    }
}
