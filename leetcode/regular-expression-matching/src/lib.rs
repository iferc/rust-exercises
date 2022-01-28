#[derive(Debug)]
pub enum Token {
    Dot,
    DotStar,
    Letter(char),
    LetterStar(char),
}

pub fn regex(pattern: &str, string: &str) -> bool {
    // this is technically unnecessary for the challenge, but gives me comfort
    if pattern.len() == 0 || string.len() == 0 {
        return false;
    }

    // make a peekable iterator so that we can look ahead to the next token
    let mut pattern_chars = pattern.chars().peekable();

    // make a container to store the tokens as they are parsed
    let mut pattern_tokens = Vec::new();
    loop {
        // iterator through the characters in the pattern manually instead of a for loop
        // this is necessary because the for loop consumes `pattern_chars` but we need to
        // reuse it to peek ahead at future characters in the string with `next_if_eq`
        let token = match (pattern_chars.next(), pattern_chars.next_if_eq(&'*')) {
            (Some('.'), Some('*')) => Token::DotStar,

            (Some('.'), _) => Token::Dot,

            (Some(pattern_char), Some('*')) => Token::LetterStar(pattern_char),

            (Some(pattern_char), _) => Token::Letter(pattern_char),

            (None, _) => break,
        };

        pattern_tokens.push(token);
    }

    // start with the first token in the pattern
    let mut pattern_tokens_iter = pattern_tokens.into_iter();
    let mut token = pattern_tokens_iter.next();

    // start with the first character in the string
    let mut string_chars = string.chars();
    let mut string_char = match string_chars.next() {
        Some(string_char) => string_char,
        None => return false,
    };

    loop {
        match token {
            // just advance to the next token since a dot matches any input
            Some(Token::Dot) => {
                token = pattern_tokens_iter.next();
            }

            Some(Token::DotStar) => {
                // todo: this is too greedy right now
            }

            Some(Token::Letter(token_char)) => {
                // fail if the character doesn't match
                if token_char != string_char {
                    return false;
                }

                // then iterate to the next token
                token = pattern_tokens_iter.next();
            }

            // only iterate to the next token when a match is not found
            Some(Token::LetterStar(token_char)) => {
                if token_char != string_char {
                    // when a match is not found, iterate to the next token
                    token = pattern_tokens_iter.next();

                    // but _not_ the next character in the string as we
                    // still need to see if the next thing matches
                    continue;
                }
            }

            // ran out of tokens before all characters in the string were processed
            None => return false,
        }

        // manually iterate through characters so that we can occasionally
        // skip advancing to the next character in the input string
        string_char = match string_chars.next() {
            Some(string_char) => string_char,
            None => break,
        };
    }

    // returns true so long as all tokens were consumed or are star matches
    // since they can match 0 occurences
    0 == pattern_tokens_iter
        .filter_map(|token| match token {
            Token::DotStar | Token::LetterStar(_) => None,
            _ => Some(token),
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::regex;

    #[test]
    fn example_1() {
        let result = regex("a", "aa");
        assert_eq!(result, false);
    }

    #[test]
    fn example_2() {
        let result = regex("a*", "aa");
        assert_eq!(result, true);
    }

    #[test]
    fn example_3() {
        let result = regex(".*", "ab");
        assert_eq!(result, true);
    }

    #[test]
    fn example_4() {
        let result = regex("a*a", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn example_5() {
        let result = regex("a*b", "aaaaaab");
        assert_eq!(result, true);
    }

    #[test]
    fn example_6() {
        let result = regex("a*b", "b");
        assert_eq!(result, true);
    }

    #[test]
    fn example_7() {
        let result = regex("a*b", "a");
        assert_eq!(result, false);
    }

    #[test]
    fn example_8() {
        let result = regex(".*b", "xyz");
        assert_eq!(result, false);
    }

    #[test]
    fn example_9() {
        let result = regex("a*.", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn example_10() {
        let result = regex("a*b*c*", "aaaaabccc");
        assert_eq!(result, true);
    }

    #[test]
    fn example_11() {
        let result = regex("a*b*c*", "aacc");
        assert_eq!(result, true);
    }

    #[test]
    fn example_12() {
        let result = regex("a*a*", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn example_13() {
        let result = regex("a*b*", "a");
        assert_eq!(result, true);
    }
}
