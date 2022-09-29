// #[deprecated(since = "0.1.0", note = "use ??? instead.")]
pub fn foo() -> &'static str {
    "hello! This is foo!"
}
#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
