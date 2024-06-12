#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn test_get_tomls() {
        let tomls = util::tomls(".").expect("file not found.");
        assert_eq!(1, tomls.len());
    }
}
