

use hashbrown::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
enum TokenType {
    LeftParen,
    RightParen,
    Pound,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    SlashSlash,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    Assert,
    If,
    Not,
    Else,
    Elif,
    While,
    For,
    Integer,
    Float,
    Complex,
    Class,

    Identifier,

}

impl From<TokenType> for &str {
    fn from(val: TokenType) -> Self {
        match val {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::Pound => "#",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::StarStar => "**",
            TokenType::Slash => "/",
            TokenType::SlashSlash => "//",
            TokenType::EqualEqual => "==",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::NotEqual => "!=",

            TokenType::Assert => "assert",
            TokenType::If => "if",
            TokenType::Not => "not",
            TokenType::Else => "else",
            TokenType::Elif => "elif",
            TokenType::While => "while",
            TokenType::For => "for",
            TokenType::Integer => "int",
            TokenType::Float => "float",
            TokenType::Complex => "complex",
            TokenType::Class => "class",

            TokenType::Identifier => "Identifier"
        }
    }
}

#[derive(Debug, Clone)]
struct KeywordTree {
    chr: char,
    token: Option<TokenType>,
    children: HashMap<char, KeywordTree>
}

impl<'source> KeywordTree {
    fn add_token(&mut self, tokentype: TokenType) {
        let mut current = self;

        let tokentype_str: &str = tokentype.into();
        for chr in tokentype_str.chars() {
            let maybe_child = current.children.get_mut(&chr);
            if maybe_child.is_none() {
                current.children.insert(chr, KeywordTree{ chr, token: None, children: HashMap::new() });
            }
            current = current.children.get_mut(&chr).expect("should't be None")
        }
        current.token = Some(tokentype);
    }

    fn populate() -> KeywordTree {
        let mut kwt = KeywordTree { chr: '\\', token: None, children: HashMap::new()};

        for tokentype in TokenType::iter() {
            kwt.add_token(tokentype)
        }        
        
        kwt
    }

    fn get_tokentype(self, token: &str) -> Option<KeywordTree> {
        let mut current = Some(self);
        for chr in token.chars() {
            current.as_ref()?;
            current = current.unwrap().children.get(&chr).cloned();
        }
        current
    }
}


#[inline]
fn is_underscore(c: char) -> bool {
    c == '_'
}

#[inline]
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

#[inline]
fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase()
}


#[derive(Debug, Clone, Copy)]
struct Token {
    tokentype: TokenType,
    start: usize,
    length: usize,
    line: usize
}

#[derive(Debug)]
pub struct Tokenizer {
    source: String,
    chars: Vec<char>,
    current_index: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Tokenizer {
        let chars: Vec<char> = source.chars().collect();
        Tokenizer { source: String::from(source), chars, current_index: 0, line: 0 }
    }

    #[inline]
    fn peek(&self, distance: usize) -> Option<char> {
        self.chars.get(self.current_index + distance).copied()
    }

    // fn at_end(&self, distance: usize) -> bool {
    //     self.current_index + distance >= self.chars.len()
    // }

    // fn num_remaining_chars(&self) -> usize {
    //     self.chars.len() - self.current_index
    // }

    fn parse_number(&self) -> Token {
        let mut found_dot = false;
        let mut found_j = false;
        let mut length: usize = 1;

        let mut chars = self.chars[self.current_index..].iter();
        let mut chr = chars.next();

        // initial digits
        while chr.is_some_and(|&c| is_digit(c) || is_underscore(c)) {
            length += 1;
            chr = chars.next()
        }
        
        // optional dot and rest of digits
        if chr.is_some_and(|&c| c == '.') {
            found_dot = true;
            length += 1;
            chr = chars.next();
            while chr.is_some_and(|&c| is_digit(c) || is_underscore(c)) {
                length += 1;
                chr = chars.next();
            }    
        }

        // optional scientific notation part
        if chr.is_some_and(|&c| c == 'e' || c == 'E') {
            length += 1;
            chr = chars.next();

            // optional +\-
            if chr.is_some_and(|&c| c == '+' || c == '-') {
                length += 1;
                chr = chars.next();
            }

            while chr.is_some_and(|&c| is_digit(c) || is_underscore(c)) {
                length += 1;
                chr = chars.next();
            }
        }

        // optional complex char
        if chr.is_some_and(|&c| c == 'j' || c == 'J') {
            found_j = true;
            length += 1;
        }

        let tokentype = if found_j {
            TokenType::Complex
        } else if found_dot {
            TokenType::Float
        } else {
            TokenType::Integer
        };

        Token {tokentype, start: self.current_index, length, line: self.line}
    }

    fn parse_reserved(&self) -> Option<Token> {
        let mut kwt_node = &KeywordTree::populate();
        let mut length: usize = 0;

        let mut chr = self.peek(length);
        while chr.is_some_and(|c| is_digit(c) || is_underscore(c)) {
            let tmp = kwt_node.children.get(&chr.unwrap());
            match tmp {
                None => break,
                Some(x) => kwt_node = x
            };

            length += 1;
            chr = self.peek(length);

        }

        if length == 0 || kwt_node.token.is_none() {
            None
        } else {
            // let keyword: String = self.chars[self.current_index..self.current_index + length].iter().collect();
            // let keyword: &str = &keyword.as_str();
            Some(Token {tokentype: kwt_node.token.unwrap(), start: self.current_index, length, line: self.line})
        }
        
    }
    
    fn parse_identifier(&self) -> Token {
        let mut length: usize = 1;
        let mut chars = self.chars[self.current_index + 1..].iter();
        let mut chr = chars.next();

        while chr.is_some_and(|&c| is_digit(c) || is_alpha(c) || is_underscore(c)) {
            chr = chars.next();
            length += 1;
        }

        Token {tokentype: TokenType::Identifier, start: self.current_index, length, line: self.line}
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut chars = self.chars[self.current_index..].iter();
        let mut chr = chars.next();
        let mut tokens: Vec<Token> = vec![];

        while chr.is_some() {
            println!("{:?}", chr);
            match *chr.unwrap() {
                // non-newline whitespace
                c if c == ' ' || c == '\t' => {
                    chr = chars.next();
                },
                // newline whitespace
                c if c == '\n' => {
                    self.line += 1;
                    chr = chars.next();
                },
                // numbers
                c if is_digit(c) => {
                    let token = self.parse_number();
                    tokens.push(token);
                    for s in 0..token.length {
                        println!("{}", s);
                        chr = chars.next();
                    }
                },
                // identifiers
                c if is_alpha(c) || is_underscore(c) => {
                    let reserved = self.parse_reserved();
                    
                    match reserved {
                        Some(t) => {
                            tokens.push(t);
                            for _ in 0..t.length {
                                chr = chars.next();
                            }                        }
                        None => {
                            let identifier = self.parse_identifier();
                            tokens.push(identifier);
                            for _ in 0..identifier.length {
                                chr = chars.next();
                            }                        }
                    }
                },
                // operators
                _ => {
                    let reserved = self.parse_reserved();
                    match reserved {
                        Some(t) => {
                            tokens.push(t);
                            for _ in 0..t.length {
                                chr = chars.next();
                            }                        },
                        None => panic!("unknown token")
                    }
                }
            };
       }

        tokens

    }

}

#[cfg(test)]
mod tests {
    use super::Tokenizer;

    #[test]
    fn basic_tokens() {
        let tokens = Tokenizer::new("3 + 1").tokenize();
        println!("{:?}", tokens)
    }
}