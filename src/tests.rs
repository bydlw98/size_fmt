use super::*;

#[test]
fn test_size_fmt_4096() {
    inner_test_size_fmt(4096_u16, "4096", "4.0K", "4.1k", "4.0Ki")
}

#[test]
fn test_size_fmt_negative_4096() {
    inner_test_size_fmt(-4096_i16, "-4096", "-4.0K", "-4.1k", "-4.0Ki")
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
