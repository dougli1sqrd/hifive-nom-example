use alloc::{string::String, vec::Vec};


enum Token<'a> {
    Num(u16),
    Command(&'a str),
    Param(&'a str),
    Symbol(&'a str),
    Equals,
    Comma,
    OpenBracket,
    CloseBracket,
}

fn tokenize<'a>(str: String) -> Vec<Token<'a>> {
    Vec::new()
}

