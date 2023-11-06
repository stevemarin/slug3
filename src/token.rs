
enum TokenType {
    // parens
    LeftParen,
    RIGHT_PAREN,
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
    POUND,

    // symbols
    PLUS,
    // PLUS_EQUAL,
    MINUS,
    // MINUS_EQUAL,
    STAR,
    // STAR_EQUAL,
    STAR_STAR,
    SLASH,
    SLASH_SLASH,
    // EQUAL,
    EQUAL_EQUAL,
    NOT_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,

    // whitespace
    SPACE,
    TAB,
    NEWLINE,

    // control flow

    // types
    INTEGER,
    FLOAT,

    // statements
    ASSERT,

    // other
    IDENTIFIER,
    EOF

}

// struct Token {
//     tokentype: TokenType,
//     literal: char,
//     start: i32,
//     line: i32
// }