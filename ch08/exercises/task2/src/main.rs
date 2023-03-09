fn pig_latin_converter(sentence: &str) -> String{
    let vowels = "aeiou";

    let mut pig_latin_sentence = String::new();
    for word in sentence.split_whitespace() {
        let (first, rest) = word.split_at(1);
        if vowels.contains(first) {
            pig_latin_sentence += &format!("{}-hay ", word).to_string();
        } else {
            pig_latin_sentence += &format!("{}-{}ay ", rest, first).to_string();
        }
    }

    let _last_char = pig_latin_sentence.pop();
    return pig_latin_sentence;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::pig_latin_converter;

    #[test]
    fn test_pig_latin_converter() {
        assert_eq!(pig_latin_converter("apple"), "apple-hay");
        assert_eq!(pig_latin_converter("first"), "irst-fay");
        assert_eq!(pig_latin_converter("apple first"), "apple-hay irst-fay");
    }
}
