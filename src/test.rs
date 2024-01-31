#[cfg(test)]
mod tests {

    use anyhow::Error;
    use libvorpal::*;

    use crate::run;

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

    #[test]
    // This uses the model id to verify that get_first is indeed getting the right
    // model.
    fn get_first_test() {
        let model_name = "cat".to_string();
        let safe = false;
        let expected_outcome = "20086".to_string();
        let queryitem = get_first_query_item(model_name, safe);
        assert_eq!(expected_outcome, queryitem.get_id());
    }
    #[test]
    // This test makes a query of 100 to ensure that Vorpal is able to make queryitems
    // properly. Previously, issues came about due to improper labeling, unicode breaking
    // things, etc. This test is necessary to ensure stability and integrity.
    fn broad_query_test_1() {
        let model_name = "cat".to_string();
        let safe = true;
        let count = 100;
        let query = get_query_items(model_name, count, safe);
        let len = query.len();
        assert_eq!(count, len as u8);
    }
    #[test]
    fn broad_query_test_2() {
        let model_name = "dog".to_string();
        let safe = true;
        let count = 100;
        let query = get_query_items(model_name, count, safe);
        let len = query.len();
        assert_eq!(count, len as u8);
    }
    #[test]
    fn broad_query_test_3() {
        let model_name = "painting".to_string();
        let safe = true;
        let count = 100;
        let query = get_query_items(model_name, count, safe);
        let len = query.len();
        assert_eq!(count, len as u8);
    }
    #[test]
    fn broad_query_test_4() {
        let model_name = "girl".to_string();
        let safe = true;
        let count = 100;
        let query = get_query_items(model_name, count, safe);
        let len = query.len();
        assert_eq!(count, len as u8);
    }

    //TODO separate main/cli tests and lib tests
    #[cfg(test)]
    use crate::Args;
    fn download_first_test() -> Result<(), Error> {
        let model_name = "cat".to_string();
        let args = Args {
            model_name: Some(
                model_name,
            ),
            get_first: false,
            directory: None,
            only_model: false,
            meta: false,
            query: None,
            count: 15,
            safe: false,
            full: false,
            url: None,
        };
        run(args)
        //assert_eq!(result, Ok(()));
        //let query = download(model_name, count, safe);
        //let len = query.len();
    }
}
