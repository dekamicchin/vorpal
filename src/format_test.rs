#[cfg(test)]
mod tests {

    use crate::format::shorten;

    #[test]
    fn shorten_test() {
        let mut input = "yeyeyeyeyeyeyeyeyeyeyeyeyeyeye".to_string();
        let trunc_length = 6;
        let trunc_trail = "...";
        let expected_outcome = "yeyeye...";

        assert_eq!(expected_outcome, shorten(input, trunc_length, trunc_trail));
    }
}

