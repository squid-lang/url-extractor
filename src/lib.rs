#![feature(inclusive_range_syntax)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate url;

use regex::Regex;
use url::Url;
use std::slice;

lazy_static! {
    static ref PROTOCOL_MATCHER: Regex = Regex::new(r"(http|https)://").unwrap();
}

#[derive(Debug, Copy, Clone)]
enum BracketType {
    Paren,
    Bracket,
    Brace,
    Angled,
}

impl BracketType {
    fn iter() -> slice::Iter<'static, BracketType> {
        const VALUES: [BracketType; 4] = [
            BracketType::Paren,
            BracketType::Bracket,
            BracketType::Brace,
            BracketType::Angled,
        ];

        VALUES.into_iter()
    }
}

impl BracketType {
    fn opener(&self) -> char {
        match *self {
            BracketType::Paren => '(',
            BracketType::Bracket => '[',
            BracketType::Brace => '{',
            BracketType::Angled => '<',
        }
    }

    fn closer(&self) -> char {
        match *self {
            BracketType::Paren => ')',
            BracketType::Bracket => ']',
            BracketType::Brace => '}',
            BracketType::Angled => '>',
        }
    }
}

fn find_first_unbalanced(input: &str, bracket_type: BracketType) -> Option<usize> {
    let mut stack = Vec::<usize>::new();
    let opener = bracket_type.opener();
    let closer = bracket_type.closer();

    for (i, c) in input.chars().enumerate() {
        if c == opener {
            stack.push(i);
        }

        if c == closer {
            if stack.is_empty() {
                return Some(i);
            } else {
                stack.pop();
            }
        }
    }

    if !stack.is_empty() {
        return Some(stack[0]);
    }

    None
}

///
/// Attempts to find an URL in this string
/// This function assumes that this string does not contain whitespaces
///
pub fn extract_url(input: &str) -> Option<(usize, usize)> {
    if input.is_empty() {
        return None;
    }

    let start = PROTOCOL_MATCHER.find(input)?.start();

    let first_unbalanced = BracketType::iter()
        .filter_map(|bracket| find_first_unbalanced(&input[start..], *bracket))
        .min()
        .map(|val| start + val)
        .unwrap_or(input.len());

    let end = first_unbalanced - 1;

    match Url::parse(&input[start..=end]) {
        Ok(url) => {
            if url.host_str()?.is_empty() {
                return None;
            }

            Some((start, end))
        }
        Err(_) => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(None, extract_url("()"));
        assert_eq!(None, extract_url("http://"));
        assert_eq!(None, extract_url("https://"));
        assert_eq!(Some((0, 9)), extract_url("https://()"));
        assert_eq!(Some((0, 12)), extract_url("http://google"));
        assert_eq!(Some((0, 13)), extract_url("https://google"));

        assert_eq!(
            Some((1, 59)),
            extract_url("(https://en.wikipedia.org/wiki/Slowloris_(computer_security))")
        );

        assert_eq!(
            Some((1, 59)),
            extract_url("[https://en.wikipedia.org/wiki/Slowloris_(computer_security)]")
        );
    }
}
