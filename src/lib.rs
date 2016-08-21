extern crate regex;

#[allow(dead_code)]
mod word_counter {
    use std::collections::BTreeMap;
    use regex::Regex;

    pub fn count(str: &str) -> BTreeMap<String, i32> {
        let mut map = BTreeMap::new();
        let re = Regex::new(r"[ _.#,:!&@$%^&]+").unwrap();
        let words = re.split(str);

        for word in words.filter(|c| !c.is_empty()) {
            let word = word.to_lowercase();

            let value = match map.get(&word) {
                Some(val) => val + 1,
                None => 1,
            };

            map.insert(word, value);
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::word_counter;
    use std::collections::BTreeMap;

    #[test]
    fn one_word_once() {
        let str = "test";

        let mut map = BTreeMap::new();
        map.insert(str.to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn multiple_words_once() {
        let str = "each one once";

        let mut map = BTreeMap::new();
        map.insert("each".to_string(), 1);
        map.insert("one".to_string(), 1);
        map.insert("once".to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn count_multiple_occurrences() {
        let str = "one fish two fish red fish blue fish";

        let mut map = BTreeMap::new();
        map.insert("one".to_string(), 1);
        map.insert("fish".to_string(), 4);
        map.insert("two".to_string(), 1);
        map.insert("red".to_string(), 1);
        map.insert("blue".to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn ignore_punctuation() {
        let str = "car : carpet as java : javascript!!&@$%^&";

        let mut map = BTreeMap::new();
        map.insert("car".to_string(), 1);
        map.insert("carpet".to_string(), 1);
        map.insert("as".to_string(), 1);
        map.insert("java".to_string(), 1);
        map.insert("javascript".to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn includes_numbers() {
        let str = "testing, 1, 2 testing";

        let mut map = BTreeMap::new();
        map.insert("testing".to_string(), 2);
        map.insert("1".to_string(), 1);
        map.insert("2".to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn normalize_case() {
        let str = "go Go GO";

        let mut map = BTreeMap::new();
        map.insert("go".to_string(), 3);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn prefix_punctuation() {
        let str = "!%%#testing, 1, 2 testing";

        let mut map = BTreeMap::new();
        map.insert("testing".to_string(), 2);
        map.insert("1".to_string(), 1);
        map.insert("2".to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn symbols_are_separators() {
        let str = "hey,my_spacebar_is_broken.";

        let mut map = BTreeMap::new();
        map.insert("hey".to_string(), 1);
        map.insert("my".to_string(), 1);
        map.insert("spacebar".to_string(), 1);
        map.insert("is".to_string(), 1);
        map.insert("broken".to_string(), 1);

        assert_eq!(word_counter::count(str), map);
    }
}
