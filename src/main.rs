//! # quine
//!
//! A quine in rust, written by wwendler

fn main() {
    program("{", "}", "\"", "\\");
    program("{open_bracket}", "{close_bracket}", "{quote}", "{backslash}");
    println!("\",
        open_bracket = open_bracket,
        close_bracket = close_bracket,
        quote = quote,
        backslash = backslash);
}}");
}

fn program(open_bracket : &str, close_bracket : &str, quote : &str, backslash : &str) {
    print!("//! # quine
//!
//! A quine in rust, written by wwendler

fn main() {open_bracket}
    program({quote}{open_bracket}{quote}, {quote}{close_bracket}{quote}, {quote}{backslash}{quote}{quote}, {quote}{backslash}{backslash}{quote});
    program({quote}{open_bracket}open_bracket{close_bracket}{quote}, {quote}{open_bracket}close_bracket{close_bracket}{quote}, {quote}{open_bracket}quote{close_bracket}{quote}, {quote}{open_bracket}backslash{close_bracket}{quote});
    println!({quote}{backslash}{quote},
        open_bracket = open_bracket,
        close_bracket = close_bracket,
        quote = quote,
        backslash = backslash);
{close_bracket}{close_bracket}{quote});
{close_bracket}

fn program(open_bracket : &str, close_bracket : &str, quote : &str, backslash : &str) {open_bracket}
    print!({quote}",
        open_bracket = open_bracket,
        close_bracket = close_bracket,
        quote = quote,
        backslash = backslash);
}
