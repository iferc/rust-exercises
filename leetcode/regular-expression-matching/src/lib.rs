#[derive(Copy, Clone, Debug)]
pub enum Token {
    Any,
    AnyStar,
    Letter(char),
    LetterStar(char),
}

pub fn is_match(input_tokens: &[Token], string: &str) -> bool {
    // start with the first token in the pattern
    let mut token_count_remaining = input_tokens.len();
    let mut tokens = input_tokens.iter();
    let mut token = tokens.next();

    // start with the first character in the string
    let mut string_count_remaining = string.len();
    let mut string_chars = string.chars();
    let mut string_char = match string_chars.next() {
        Some(string_char) => string_char,
        None => return false,
    };

    loop {
        match token {
            // just advance to the next token since any letter can match
            Some(Token::Any) => {
                token = tokens.next();
                token_count_remaining -= 1;
            }

            Some(Token::AnyStar) => {
                // recursively call self with the next remaining tokens and string
                if is_match(
                    &input_tokens[input_tokens.len() - token_count_remaining + 1..],
                    &string[string.len() - string_count_remaining..],
                ) {
                    return true;
                }
            }

            Some(Token::Letter(token_char)) => {
                // fail if the character doesn't match
                if token_char != &string_char {
                    return false;
                }

                // then iterate to the next token
                token = tokens.next();
                token_count_remaining -= 1;
            }

            // only iterate to the next token when a match is not found
            Some(Token::LetterStar(token_char)) => {
                // recursively call self with the next remaining tokens and string
                if is_match(
                    &input_tokens[input_tokens.len() - token_count_remaining + 1..],
                    &string[string.len() - string_count_remaining..],
                ) {
                    return true;
                }

                if token_char != &string_char {
                    // when a match is not found, iterate to the next token
                    token = tokens.next();
                    token_count_remaining -= 1;

                    // but _not_ the next character in the string as we
                    // still need to see if the next thing matches
                    continue;
                }
            }

            // ran out of tokens before all characters in the string were processed
            None => {
                return false;
            }
        }

        // manually iterate through characters so that we can occasionally
        // skip advancing to the next character in the input string
        string_char = match string_chars.next() {
            Some(string_char) => {
                string_count_remaining -= 1;
                string_char
            }
            None => break,
        };
    }

    // returns true so long as all tokens were consumed or are star matches
    // since they can match 0 occurences
    0 == match token {
        Some(Token::AnyStar) | Some(Token::LetterStar(_)) | None => 0,
        Some(_) => 1,
    } + tokens.fold(0, |sum, token| match token {
        Token::AnyStar | Token::LetterStar(_) => sum,
        _ => sum + 1,
    })
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
            (Some('.'), Some('*')) => Token::AnyStar,

            (Some('.'), _) => Token::Any,

            (Some(pattern_char), Some('*')) => Token::LetterStar(pattern_char),

            (Some(pattern_char), _) => Token::Letter(pattern_char),

            (None, _) => break,
        };

        pattern_tokens.push(token);
    }

    // is match
    is_match(pattern_tokens.as_slice(), string)
}

#[cfg(test)]
mod tests {
    use super::regex;

    /// Example 1 from ReadMe
    #[test]
    fn simple_text() {
        let result = regex("a", "aa");
        assert_eq!(result, false);
    }

    /// Example 2 from ReadMe
    #[test]
    fn wildcard_letter_pattern() {
        let result = regex("a*", "aa");
        assert_eq!(result, true);
    }

    /// Example 3 from ReadMe
    #[test]
    fn wildcard_pattern() {
        let result = regex(".*", "ab");
        assert_eq!(result, true);
    }

    #[test]
    fn wildcard_prefix_not_blocking_letter() {
        let result = regex("a*a", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn wildcard_prefix_before_match_1() {
        let result = regex("a*b", "aaaaaab");
        assert_eq!(result, true);
    }

    #[test]
    fn wildcard_prefix_before_match_2() {
        let result = regex("a*b", "b");
        assert_eq!(result, true);
    }

    #[test]
    fn wildcard_prefix_not_matching_different_remainder() {
        let result = regex("a*b", "a");
        assert_eq!(result, false);
    }

    #[test]
    fn any_wildcard_prefix_not_blocking_pattern_remainder() {
        let result = regex(".*b", "xyz");
        assert_eq!(result, false);
    }

    #[test]
    fn wildcard_prefix_before_any_match() {
        let result = regex("a*.", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn multi_wildcard() {
        let result = regex("a*b*c*", "aaaaabccc");
        assert_eq!(result, true);
    }

    #[test]
    fn multi_wildcard_with_some_not_matching() {
        let result = regex("a*b*c*", "aacc");
        assert_eq!(result, true);
    }

    #[test]
    fn dual_wildcards() {
        let result = regex("a*a*", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn dual_wildcards_with_only_one_matching() {
        let result = regex("a*b*", "a");
        assert_eq!(result, true);
    }

    #[test]
    fn maximum_possible_recursion() {
        let result = regex("a*b*c*d*e*f*g*h*i*j*k*l*m*n*o*", "aaaaaaaaaaaaaaaaaaaa");
        assert_eq!(result, true);
    }

    #[test]
    fn wildcard_prefix_on_max_anys_to_match_input() {
        let result = regex(".*....................", "aaaaaaaaaaaaaaaaaaaa");
        assert_eq!(result, true);
    }

    #[test]
    fn longer_pattern_than_possible_string() {
        let result = regex("....................", "aaaaaaaaaaaaaaaaaaaa");
        assert_eq!(result, true);
    }

    #[test]
    fn simple_letters_shorter_than_pattern() {
        let result = regex("aaaa", "aaa");
        assert_eq!(result, false);
    }

    #[test]
    fn simple_letters_longer_than_pattern() {
        let result = regex("aaa", "aaaa");
        assert_eq!(result, false);
    }

    #[test]
    fn mixed_wildcards_and_any_matches() {
        let result = regex(".*..a*", "a");
        assert_eq!(result, false);
    }

    #[test]
    fn misleading_wildcard() {
        let result = regex("a*.*b.a.*c*b*a*c*", "abbabaaaaaaacaa");
        assert_eq!(result, true);
    }
}
