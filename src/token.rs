
enum TokenType {
    // parens
    LeftParen,
    RightParen,
    // LEFT_SQUARE,
    // RIGHT_SQUARE,
    // LEFT_BRACE,
    // RIGHT_BRACE,

    // quotes
    // SINGLE_QUOTE,
    // DOUBLE_QUOTE,
    // TRIPLE_SINGLE_QUOTE,
    // TRIPLE_DOUBLE_QUOTE,

    // comments
    Pound,

    // symbols
    Plus,
    // PLUS_EQUAL,
    Minus,
    // MINUS_EQUAL,
    Star,
    // STAR_EQUAL,
    StarStar,
    Slash,
    SlashSlash,
    // EQUAL,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // whitespace
    Space,
    Tab,
    Newline,

    // control flow

    // types
    Integer,
    Float,

    // statements
    ASSERT,

    // other
    Identifier,
    Eof

}

// struct Token {
//     tokentype: TokenType,
//     literal: char,
//     start: i32,
//     line: i32
// }