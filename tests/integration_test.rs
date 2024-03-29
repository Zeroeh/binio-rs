// use binio;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[test]
fn test_new() {
    let mut x = binio::new_buffer(123);
    x.junk_fill();
}

#[test]
fn test_floats() {
    let r = 3.53333;
    let mut x = binio::new_buffer(8);
    x.write_f64(r);
    x.index = 0;
    let f = x.read_f64();
    assert_eq!(r, f);
}

#[test]
fn test_strings() {
    let r = String::from("rotmg");
    let mut x = binio::new_buffer(7);
    x.write_string(r.clone());
    x.index = 0;
    let f = x.read_string();
    assert_eq!(r, f);
}

#[test]
fn test_signed_unsigned() {
    let r = -1;
    let mut x = binio::new_buffer(0);
    x.write_i32(r);
    x.index = 0;
    let f = x.read_i32();
    assert_eq!(r, f);
}
