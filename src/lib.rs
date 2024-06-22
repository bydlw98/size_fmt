#![no_std]
#[doc = include_str!("../README.md")]
mod size_fmt_impl;

#[cfg(test)]
mod tests;

pub use size_fmt_impl::Integer;

#[derive(Default)]
pub struct Buffer {
    inner: size_fmt_impl::Buffer,
}

impl Buffer {
    pub fn new() -> Self {
        let inner = size_fmt_impl::Buffer::new();

        Self { inner }
    }

    pub fn raw_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.raw_fmt(size)
    }

    pub fn human_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.human_fmt(size)
    }

    pub fn si_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.si_fmt(size)
    }

    pub fn iec_fmt<I: Integer>(&mut self, size: I) -> &str {
        self.inner.iec_fmt(size)
    }
}
