#![feature(inclusive_range_syntax)]

extern crate url_extractor;

use url_extractor::extract_url;

fn main() {
    let input = "(https://en.wikipedia.org/wiki/Slowloris_(computer_security))";
    let (start, end) = extract_url(&input).unwrap();

    println!("{}", &input[start..=end]);
}
