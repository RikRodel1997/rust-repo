use crate::errors::{Error, ErrorKind};

pub const WHITESPACES: [char; 3] = [' ', '\r', '\n'];
pub const DELIMITERS: [char; 8] = [' ', '\r', '\n', ':', '=', '(', ')', ','];

pub fn lex(source: &str) -> Result<Vec<&str>, Error> {
    if source.is_empty() {
        return Err(Error {
            kind: ErrorKind::Empty,
            message: "Can't lex empty input".to_string(),
        });
    }

    let mut tokens: Vec<&str> = Vec::new();
    let mut s = source;

    while let Some(char) = s.chars().next() {
        let is_whitespace = WHITESPACES.contains(&char);
        let is_delimiter = DELIMITERS.contains(&char);
        let char_size = char.len_utf8();

        if is_whitespace {
            s = &s[char_size..];
            continue;
        }

        if is_delimiter {
            tokens.push(&s[..char_size]);
            s = &s[char_size..];
        } else {
            let end = s
                .find(|inner_char: char| {
                    inner_char.is_whitespace() || DELIMITERS.contains(&inner_char)
                })
                .unwrap_or(s.len());
            tokens.push(&s[..end]);
            s = &s[end..];
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let tokens = lex(
            "a : integer = 0\na := 0\n\nb : integer\nb := 0\n\ndefun foo (a:integer, b:integer):integer {\n\n}",
        );
        let expected = vec![
            "a", ":", "integer", "=", "0", "a", ":", "=", "0", "b", ":", "integer", "b", ":", "=",
            "0", "defun", "foo", "(", "a", ":", "integer", ",", "b", ":", "integer", ")", ":",
            "integer", "{", "}",
        ];
        assert!(tokens.is_ok());

        let actual = tokens.unwrap();
        assert_eq!(actual, expected);
    }
}
