use super::*;

#[test]
fn test_size_fmt_4096_u16() {
    inner_test_size_fmt(4096_u16, "4096", "4.0K", "4.1k", "4.0Ki");
}

#[test]
fn test_size_fmt_negative_4096_i16() {
    inner_test_size_fmt(-4096_i16, "-4096", "-4.0K", "-4.1k", "-4.0Ki");
}

#[test]
fn test_size_fmt_1000_u32() {
    inner_test_size_fmt(1000_u32, "1000", "1000", "1.0k", "1000");
}

#[test]
fn test_size_fmt_100_000_u32() {
    inner_test_size_fmt(100_000_u32, "100000", "98K", "100k", "98Ki");
}

#[test]
fn test_size_fmt_2_000_000_u32() {
    inner_test_size_fmt(2_000_000_u32, "2000000", "2.0M", "2.0M", "2.0Mi");
}

#[test]
fn test_size_fmt_200_000_000_u32() {
    inner_test_size_fmt(200_000_000_u32, "200000000", "191M", "200M", "191Mi");
}

#[test]
fn test_size_fmt_2_000_000_000_u32() {
    inner_test_size_fmt(2_000_000_000_u32, "2000000000", "1.9G", "2.0G", "1.9Gi");
}

fn inner_test_size_fmt<I: Integer>(
    size: I,
    raw_str: &str,
    human_str: &str,
    si_str: &str,
    iec_str: &str,
) {
    let mut buf = Buffer::new();
    assert_eq!(buf.raw_fmt(size), raw_str);

    let mut buf = Buffer::new();
    assert_eq!(buf.human_fmt(size), human_str);

    let mut buf = Buffer::new();
    assert_eq!(buf.si_fmt(size), si_str);

    let mut buf = Buffer::new();
    assert_eq!(buf.iec_fmt(size), iec_str);
}
