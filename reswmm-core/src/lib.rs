#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_str<'a>(s:&'a str) -> &'a str {
    return s;
}
