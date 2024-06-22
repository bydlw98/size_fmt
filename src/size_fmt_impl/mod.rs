#[macro_use]
mod macros;

use core::mem::{self, MaybeUninit};
use core::ptr;
use core::slice;
use core::str;

const ITOA_BUFFER_LEN: usize = mem::size_of::<itoa::Buffer>();

pub union Buffer {
    itoa_buf: itoa::Buffer,
    bytes: [MaybeUninit<u8>; ITOA_BUFFER_LEN],
}

impl Buffer {
    pub fn new() -> Self {
        let bytes = [MaybeUninit::<u8>::uninit(); ITOA_BUFFER_LEN];

        Self { bytes }
    }

    pub fn raw_fmt<I: Integer>(&mut self, size: I) -> &str {
        size.raw_write(self)
    }

    pub fn human_fmt<I: Integer>(&mut self, size: I) -> &str {
        size.human_write(self)
    }

    pub fn si_fmt<I: Integer>(&mut self, size: I) -> &str {
        size.si_write(self)
    }

    pub fn iec_fmt<I: Integer>(&mut self, size: I) -> &str {
        size.iec_write(self)
    }

    fn to_str(&self, len: usize) -> &str {
        let bytes = unsafe { slice::from_raw_parts(self.bytes.as_ptr() as *const u8, len) };

        unsafe { str::from_utf8_unchecked(bytes) }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Integer: itoa::Integer + private::Sealed {}

mod private {
    pub trait Sealed: Copy {
        fn raw_write(self, buf: &mut super::Buffer) -> &str;

        fn human_write(self, buf: &mut super::Buffer) -> &str;

        fn si_write(self, buf: &mut super::Buffer) -> &str;

        fn iec_write(self, buf: &mut super::Buffer) -> &str;

        fn inner_write<'buf>(
            self,
            factor: Self,
            prefixs: &'static [&'static str],
            buf: &'buf mut super::Buffer,
        ) -> &'buf str;
    }
}

impl Integer for u8 {}
impl private::Sealed for u8 {
    fn raw_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }

    fn human_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }

    fn si_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }

    fn iec_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }

    fn inner_write<'buf>(
        self,
        _factor: Self,
        _prefixs: &'static [&'static str],
        _buf: &'buf mut self::Buffer,
    ) -> &'buf str {
        ""
    }
}

impl Integer for i8 {}
impl private::Sealed for i8 {
    fn raw_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }
    fn human_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }
    fn si_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }
    fn iec_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }
    fn inner_write<'buf>(
        self,
        _factor: Self,
        _prefixs: &'static [&'static str],
        _buf: &'buf mut self::Buffer,
    ) -> &'buf str {
        ""
    }
}

impl Integer for u16 {}
impl private::Sealed for u16 {
    fn raw_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }

    fn human_write(self, buf: &mut self::Buffer) -> &str {
        self.inner_write(1024, &["K"], buf)
    }

    fn si_write(self, buf: &mut self::Buffer) -> &str {
        self.inner_write(1000, &["k"], buf)
    }

    fn iec_write(self, buf: &mut self::Buffer) -> &str {
        self.inner_write(1024, &["Ki"], buf)
    }

    fn inner_write<'buf>(
        self,
        factor: Self,
        prefixs: &'static [&'static str],
        buf: &'buf mut self::Buffer,
    ) -> &'buf str {
        let mut buf_len: usize = 0;

        if self < factor {
            unsafe { buf.itoa_buf.format(self) }
        } else {
            let prefix = prefixs[0];
            fmt_with_prefix!(false, u16, self, factor, prefix, buf, buf_len);
            buf.to_str(buf_len)
        }
    }
}

impl Integer for i16 {}
impl private::Sealed for i16 {
    fn raw_write(self, buf: &mut self::Buffer) -> &str {
        unsafe { buf.itoa_buf.format(self) }
    }

    fn human_write(self, buf: &mut self::Buffer) -> &str {
        self.inner_write(1024, &["K"], buf)
    }

    fn si_write(self, buf: &mut self::Buffer) -> &str {
        self.inner_write(1000, &["k"], buf)
    }

    fn iec_write(self, buf: &mut self::Buffer) -> &str {
        self.inner_write(1024, &["Ki"], buf)
    }

    fn inner_write<'buf>(
        self,
        factor: Self,
        prefixs: &'static [&'static str],
        buf: &'buf mut self::Buffer,
    ) -> &'buf str {
        let mut buf_len: usize = 0;

        let mut is_negative = false;
        let mut self_abs = self;
        if self < 0 {
            is_negative = true;
            self_abs = -self;
        }

        if self_abs < factor {
            unsafe { buf.itoa_buf.format(self) }
        } else {
            let prefix = prefixs[0];
            fmt_with_prefix!(is_negative, i16, self_abs, factor, prefix, buf, buf_len);
            buf.to_str(buf_len)
        }
    }
}
