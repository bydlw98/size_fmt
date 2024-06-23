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

#[test]
fn test_size_fmt_1000_i32() {
    inner_test_size_fmt(1000_i32, "1000", "1000", "1.0k", "1000");
}

#[test]
fn test_size_fmt_100_000_i32() {
    inner_test_size_fmt(100_000_i32, "100000", "98K", "100k", "98Ki");
}

#[test]
fn test_size_fmt_2_000_000_i32() {
    inner_test_size_fmt(2_000_000_i32, "2000000", "2.0M", "2.0M", "2.0Mi");
}

#[test]
fn test_size_fmt_200_000_000_i32() {
    inner_test_size_fmt(200_000_000_i32, "200000000", "191M", "200M", "191Mi");
}

#[test]
fn test_size_fmt_2_000_000_000_i32() {
    inner_test_size_fmt(2_000_000_000_i32, "2000000000", "1.9G", "2.0G", "1.9Gi");
}

#[test]
fn test_size_fmt_negative_1000_i32() {
    inner_test_size_fmt(-1000_i32, "-1000", "-1000", "-1.0k", "-1000");
}

#[test]
fn test_size_fmt_negative_100_000_i32() {
    inner_test_size_fmt(-100_000_i32, "-100000", "-98K", "-100k", "-98Ki");
}

#[test]
fn test_size_fmt_negative_2_000_000_i32() {
    inner_test_size_fmt(-2_000_000_i32, "-2000000", "-2.0M", "-2.0M", "-2.0Mi");
}

#[test]
fn test_size_fmt_negative_200_000_000_i32() {
    inner_test_size_fmt(-200_000_000_i32, "-200000000", "-191M", "-200M", "-191Mi");
}

#[test]
fn test_size_fmt_negative_2_000_000_000_i32() {
    inner_test_size_fmt(
        -2_000_000_000_i32,
        "-2000000000",
        "-1.9G",
        "-2.0G",
        "-1.9Gi",
    );
}

#[test]
fn test_size_fmt_1000_u64() {
    inner_test_size_fmt(1000_u64, "1000", "1000", "1.0k", "1000");
}

#[test]
fn test_size_fmt_100_000_u64() {
    inner_test_size_fmt(100_000_u64, "100000", "98K", "100k", "98Ki");
}

#[test]
fn test_size_fmt_2_000_000_u64() {
    inner_test_size_fmt(2_000_000_u64, "2000000", "2.0M", "2.0M", "2.0Mi");
}

#[test]
fn test_size_fmt_200_000_000_u64() {
    inner_test_size_fmt(200_000_000_u64, "200000000", "191M", "200M", "191Mi");
}

#[test]
fn test_size_fmt_3_000_000_000_u64() {
    inner_test_size_fmt(3_000_000_000_u64, "3000000000", "2.8G", "3.0G", "2.8Gi");
}

#[test]
fn test_size_fmt_300_000_000_000_u64() {
    inner_test_size_fmt(300_000_000_000_u64, "300000000000", "280G", "300G", "280Gi");
}

#[test]
fn test_size_fmt_4_000_000_000_000_u64() {
    inner_test_size_fmt(
        4_000_000_000_000_u64,
        "4000000000000",
        "3.7T",
        "4.0T",
        "3.7Ti",
    );
}

#[test]
fn test_size_fmt_400_000_000_000_000_u64() {
    inner_test_size_fmt(
        400_000_000_000_000_u64,
        "400000000000000",
        "364T",
        "400T",
        "364Ti",
    );
}

#[test]
fn test_size_fmt_5_000_000_000_000_000_u64() {
    inner_test_size_fmt(
        5_000_000_000_000_000_u64,
        "5000000000000000",
        "4.5P",
        "5.0P",
        "4.5Pi",
    );
}

#[test]
fn test_size_fmt_500_000_000_000_000_000_u64() {
    inner_test_size_fmt(
        500_000_000_000_000_000_u64,
        "500000000000000000",
        "445P",
        "500P",
        "445Pi",
    );
}

#[test]
fn test_size_fmt_6_000_000_000_000_000_000_u64() {
    inner_test_size_fmt(
        6_000_000_000_000_000_000_u64,
        "6000000000000000000",
        "5.3E",
        "6.0E",
        "5.3Ei",
    );
}

#[test]
fn test_size_fmt_18_000_000_000_000_000_000_u64() {
    inner_test_size_fmt(
        18_000_000_000_000_000_000_u64,
        "18000000000000000000",
        "16E",
        "18E",
        "16Ei",
    );
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
