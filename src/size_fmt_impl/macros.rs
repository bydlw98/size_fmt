macro_rules! fmt_with_prefix {
    ($is_negative:ident, $t:ty, $size:ident, $factor:ident, $prefix:ident, $buffer:ident, $buf_len:ident) => {
        unsafe {
            let mut itoa_buf = itoa::Buffer::new();
            let buf_ptr = $buffer.bytes.as_ptr() as *mut u8;
            let mut d = $size / $factor;
            let r = $size % $factor;

            if $is_negative {
                ptr::write(buf_ptr, b'-');
                $buf_len += 1;
            }

            // if d >= 10, the size format will be
            // "%d%p" where
            // %d is the ceil value of $size divided by $prefix
            // %p is the prefix
            // e.g. "400K"
            if d >= 10 {
                if r > 0 {
                    d += 1;
                }
                let size_str = itoa_buf.format(d);
                let size_len = size_str.len();
                ptr::copy_nonoverlapping(size_str.as_ptr(), buf_ptr.add($buf_len), size_len);
                $buf_len += size_len;
                ptr::copy_nonoverlapping($prefix.as_ptr(), buf_ptr.add($buf_len), $prefix.len());
                $buf_len += $prefix.len();
            }
            // else the size format will be
            // "%d.%f%p" where
            // %d is the divisor value of $size divided by $prefix
            // %f is the first digit fractional ceil value of $size divided by $prefix
            // %p is the prefix
            // e.g. "4.0K"
            else {
                let d_str = itoa_buf.format(d);
                let d_len = d_str.len();
                ptr::copy_nonoverlapping(d_str.as_ptr(), buf_ptr.add($buf_len), d_len);
                $buf_len += d_len;

                ptr::write(buf_ptr.add($buf_len), b'.');
                $buf_len += 1;

                let fract = (r as f32) / ($factor as f32);
                let fract = libm::ceilf(fract * 10.0) as u32;
                let mut itoa_buf = itoa::Buffer::new();
                let fract_str = itoa_buf.format(fract);
                let fract_len = fract_str.len();
                ptr::copy_nonoverlapping(fract_str.as_ptr(), buf_ptr.add($buf_len), fract_len);
                $buf_len += fract_len;

                ptr::copy_nonoverlapping($prefix.as_ptr(), buf_ptr.add($buf_len), $prefix.len());
                $buf_len += $prefix.len();
            }
        }
    };
}
