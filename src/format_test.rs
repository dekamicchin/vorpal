#[cfg(test)]
mod tests {

    use crate::format::shorten_unicode;

    #[test]
    fn shorten_test() {
        let input = "yeyeyeyeyeyeyeyeyeyeyeyeyeyeye".to_string();
        let trunc_length = 6;
        let trunc_trail = "...";
        let expected_outcome = "yeyeye...";

        assert_eq!(expected_outcome, shorten_unicode(input, trunc_length, trunc_trail));
    }
    #[test]
    fn shorten_japanese_test() {
        let input = "魑魅魍魎".to_string();
        let trunc_length = 2;
        let trunc_trail = "!";
        let expected_outcome = "魑魅!";

        assert_eq!(expected_outcome, shorten_unicode(input, trunc_length, trunc_trail));
    }
    #[test]
    fn shorten_emoji_test() {
        let input = "Your 📣 life literally 💯 is as valuable as a summer ant. 🐜 I'm 😏 just 👏 gonna stomp you, 👉 you're 🐶 gonna keep 🙊 coming back, 👻 I'm gonna 🙄 seal 🤐 up 😱 all 💯 my cracks, 😆 you're 😃 gonna 🗣️ keep coming back, why? 😳🤔 Cause you 👈 keep 🌵 smelling 😝👃👃💀 the syrup, you 👉🏼👴🏿 worthless b 🔥".to_string();
        let trunc_length = 41;
        let trunc_trail = "[REDACTED]";
        let expected_outcome = "Your 📣 life literally 💯 is as valuable as[REDACTED]";

        assert_eq!(expected_outcome, shorten_unicode(input, trunc_length, trunc_trail));
    }
    #[test]
    fn shorten_rune_test() {
        let input = "ᚠ feoh, ᚢ ur, ᚦ þorn, ᚩ os, ᚱ rad, ᚳ".to_string();
        let trunc_length = 12;
        let trunc_trail = "...";
        let expected_outcome = "ᚠ feoh, ᚢ ur...";

        assert_eq!(expected_outcome, shorten_unicode(input, trunc_length, trunc_trail));
    }
}

