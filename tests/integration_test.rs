extern crate binio;

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
