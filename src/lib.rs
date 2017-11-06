#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate url;

use regex::Regex;

lazy_static! {
    static ref PROTOCOL_MATCHER: Regex = Regex::new(r"(http|https)://").unwrap();
}

fn find_first_unbalanced(input: &str) -> Option<usize> {
    let mut stack = Vec::<usize>::new();
    let parens = input.chars();

    for (i, c) in parens.enumerate() {
        match c {
            '(' => stack.push(i),
            ')' => if stack.is_empty() {
                return Some(i);
            } else {
                stack.pop();
            },
            _ => {}
        };
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
    let start = PROTOCOL_MATCHER.find(input)?.start();
    let end = find_first_unbalanced(&input[start..])
        .map(|val| start + val)
        .unwrap_or(input.len());

    Some((start, end))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let position = extract_url("(https://en.wikipedia.org/wiki/Slowloris_(computer_security))");
    }
}
