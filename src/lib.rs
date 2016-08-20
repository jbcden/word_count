extern crate regex;

#[allow(dead_code)]
mod word_counter {
    use std::collections::BTreeMap;
    use regex::Regex;

    pub fn count(str: &str) -> BTreeMap<&str, i32> {
        let mut map = BTreeMap::new();
        let re = Regex::new(r"[ ,:!&@$%^&]+").unwrap();
        let words = re.split(str);

        for word in words {
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
        map.insert(str, 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn multiple_words_once() {
        let str = "each one once";

        let mut map = BTreeMap::new();
        map.insert("each", 1);
        map.insert("one", 1);
        map.insert("once", 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn count_multiple_occurrences() {
        let str = "one fish two fish red fish blue fish";

        let mut map = BTreeMap::new();
        map.insert("one", 1);
        map.insert("fish", 4);
        map.insert("two", 1);
        map.insert("red", 1);
        map.insert("blue", 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn ignore_punctuation() {
        let str = "car : carpet as java : javascript!!&@$%^&";

        let mut map = BTreeMap::new();
        map.insert("car", 1);
        map.insert("carpet", 1);
        map.insert("as", 1);
        map.insert("java", 1);
        map.insert("javascript", 1);

        assert_eq!(word_counter::count(str), map);
    }

    #[test]
    fn includes_numbers() {
        let str = "testing, 1, 2 testing";

        let mut map = BTreeMap::new();
        map.insert("testing", 2);
        map.insert("1", 1);
        map.insert("2", 1);

        assert_eq!(word_counter::count(str), map);
    }
}
