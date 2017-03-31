//! # quine
//!
//! A quine in rust, written by wwendler

fn main() {
    program("{", "}", "\"", "\\");
    program("{0}", "{1}", "{2}", "{3}");
    println!("\", open_bracket, close_bracket, quote, backslash);
}}");
}

fn program(open_bracket : &str, close_bracket : &str, quote : &str, backslash : &str) {
    print!("//! # quine
//!
//! A quine in rust, written by wwendler

fn main() {0}
    program({2}{0}{2}, {2}{1}{2}, {2}{3}{2}{2}, {2}{3}{3}{2});
    program({2}{0}0{1}{2}, {2}{0}1{1}{2}, {2}{0}2{1}{2}, {2}{0}3{1}{2});
    println!({2}{3}{2}, open_bracket, close_bracket, quote, backslash);
{1}{1}{2});
{1}

fn program(open_bracket : &str, close_bracket : &str, quote : &str, backslash : &str) {0}
    print!({2}", open_bracket, close_bracket, quote, backslash);
}
