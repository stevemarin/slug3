
use hashbrown::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
pub enum KeywordType {
    Assert,
    If,
    In,
    Not,
    Else,
    Elif,
    While,
    For,
    Integer,
    Float,
    Complex,
    Class,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
pub enum OperatorType {
    LeftParen,
    RightParen,
    Pound,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    SlashSlash,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl From<KeywordType> for &str {
    fn from(val: KeywordType) -> Self {
        match val {
            KeywordType::Assert => "assert",
            KeywordType::If => "if",
            KeywordType::In => "in",
            KeywordType::Not => "not",
            KeywordType::Else => "else",
            KeywordType::Elif => "elif",
            KeywordType::While => "while",
            KeywordType::For => "for",
            KeywordType::Integer => "int",
            KeywordType::Float => "float",
            KeywordType::Complex => "complex",
            KeywordType::Class => "class",
        }
    }
}

impl From<OperatorType> for &str {
    fn from(val: OperatorType) -> Self {
        match val {
            OperatorType::LeftParen => "(",
            OperatorType::RightParen => ")",
            OperatorType::Pound => "#",
            OperatorType::Plus => "+",
            OperatorType::Minus => "-",
            OperatorType::Star => "*",
            OperatorType::StarStar => "**",
            OperatorType::Slash => "/",
            OperatorType::SlashSlash => "//",
            OperatorType::Equal => "=",
            OperatorType::EqualEqual => "==",
            OperatorType::Less => "<",
            OperatorType::LessEqual => "<=",
            OperatorType::Greater => ">",
            OperatorType::GreaterEqual => ">=",
            OperatorType::NotEqual => "!=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword(KeywordType),
    Operator(OperatorType),
    Identifier,
    Eof
}

impl From<TokenType> for &str {
    fn from(val: TokenType) -> Self {
        match val {
            TokenType::Keyword(x) => x.into(),
            TokenType::Operator(x) => x.into(),
            TokenType::Identifier => "Identifier",
            TokenType::Eof => "Eof",
        }
    }
}

#[derive(Debug, Clone)]
struct KeywordTree {
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
                current.children.insert(chr, KeywordTree{ token: None, children: HashMap::new() });
            }
            current = current.children.get_mut(&chr).unwrap()
        }
        current.token = Some(tokentype);
    }

    fn populate() -> (KeywordTree, KeywordTree) {
        let mut kwt = KeywordTree { token: None, children: HashMap::new()};
        let mut ot = KeywordTree { token: None, children: HashMap::new()};

        for keyword in KeywordType::iter() {
            kwt.add_token(TokenType::Keyword(keyword))
        }

        for keyword in OperatorType::iter() {
            ot.add_token(TokenType::Operator(keyword))
        }
                
        (kwt, ot)
    }

    // fn get_tokentype(self, token: &str) -> Option<KeywordTree> {
    //     let mut current = Some(self);
    //     for chr in token.chars() {
    //         current = current.unwrap().children.get(&chr).cloned();
    //     }
    //     current
    // }
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub tokentype: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize
}

#[derive(Debug)]
pub struct Tokenizer<'source> {
    pub source: &'source str,
    chars: Vec<char>,
    current_index: usize,
    line: usize,
    keywords: KeywordTree,
    operators: KeywordTree,
}

impl<'source> Tokenizer<'source> {
    pub fn new(source: &str) -> Tokenizer {
        let chars: Vec<char> = source.chars().collect();
        let (keywords, operators) = KeywordTree::populate();
        Tokenizer { source, chars, current_index: 0, line: 0, keywords, operators }
    }

    // #[inline]
    fn peek(&self, distance: usize) -> Option<char> {
        self.chars.get(self.current_index + distance).copied()
    }

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
            TokenType::Keyword(KeywordType::Complex)
        } else if found_dot {
            TokenType::Keyword(KeywordType::Float)
        } else {
            TokenType::Keyword(KeywordType::Integer)
        };

        Token {tokentype, start: self.current_index, length, line: self.line}
    }

    fn parse_keywords(&self) -> Option<Token> {
        let mut keyword_node = &self.keywords;
        let mut length: usize = 0;

        let mut chr = self.peek(length);
        println!("chr: {:?}", chr);
        while chr.is_some_and(|c| is_digit(c) || is_alpha(c) || is_underscore(c)) {
            let tmp = keyword_node.children.get(&chr.unwrap());
            match tmp {
                None => break,
                Some(x) => keyword_node = x
            };

            length += 1;
            chr = self.peek(length);

        }

        if length == 0 || keyword_node.token.is_none() {
            None
        } else {
            Some(Token {tokentype: keyword_node.token.unwrap(), start: self.current_index, length, line: self.line})
        }
        
    }

    fn parse_operators(&self) -> Option<Token> {
        let mut op_node = &self.operators;
        let mut length: usize = 0;

        let mut chr = self.peek(length);
        println!("chr: {:?}", chr);
        while chr.is_some() {
            let tmp = op_node.children.get(&chr.unwrap());
            match tmp {
                None => break,
                Some(x) => op_node = x
            };

            length += 1;
            chr = self.peek(length);

        }

        if length == 0 || op_node.token.is_none() {
            None
        } else {
            Some(Token {tokentype: op_node.token.unwrap(), start: self.current_index, length, line: self.line})
        }
        
    }

    fn parse_identifier(&self) -> Token {
        let mut length: usize = 0;
        let mut chars = self.chars[self.current_index..].iter();
        let mut chr = chars.next();

        while chr.is_some_and(|&c| is_digit(c) || is_alpha(c) || is_underscore(c)) {
            chr = chars.next();
            length += 1;
        }

        Token {tokentype: TokenType::Identifier, start: self.current_index, length, line: self.line}
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        // let mut chars = self.chars[self.current_index..].iter();
        // let mut chr = chars.next();
        let mut tokens: Vec<Token> = vec![];

        let mut chr = self.peek(0);
        while chr.is_some() {
            println!("{:?}", chr);
            match chr.unwrap() {
                // non-newline whitespace
                c if c == ' ' || c == '\t' => {
                    self.current_index += 1;
                },
                // newline whitespace
                c if c == '\n' => {
                    self.line += 1;
                    self.current_index += 1;
                },
                // numbers
                c if is_digit(c) => {
                    let token = self.parse_number();
                    tokens.push(token);
                    self.current_index += token.length;              
                },
                // identifiers
                c if is_alpha(c) || is_underscore(c) => {
                    let reserved = self.parse_keywords();
                    
                    match reserved {
                        Some(t) => {
                            tokens.push(t);
                            self.current_index += t.length;              
    
                        }
                        None => {
                            let identifier = self.parse_identifier();
                            tokens.push(identifier);
                            self.current_index += identifier.length;                              
                        }
                    }
                },
                // operators
                _ => {
                    let reserved = self.parse_operators();
                    match reserved {
                        Some(t) => {
                            tokens.push(t);
                            self.current_index += t.length;
                         },
                        None => {
                            for t in tokens.into_iter() {
                                println!("\t{:?}", t);
                            }
                            panic!("unknown token");
                        }
                    }
                }
            };
            chr = self.peek(0);
       }

        tokens

    }

}

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer, KeywordType, TokenType};

    #[test]
    fn test_basic_number_parsing() {
        let tokens = Tokenizer::new(r"1.2 21 2.1J 1e-3j 1e-1").tokenize();
        let truth = vec![	
            Token { tokentype: TokenType::Keyword(KeywordType::Float), start: 0, length: 4, line: 0 },
            Token { tokentype: TokenType::Keyword(KeywordType::Integer), start: 4, length: 3, line: 0 },
            Token { tokentype: TokenType::Keyword(KeywordType::Complex), start: 7, length: 5, line: 0 },
            Token { tokentype: TokenType::Keyword(KeywordType::Complex), start: 12, length: 6, line: 0 },
            Token { tokentype: TokenType::Keyword(KeywordType::Integer), start: 18, length: 5, line: 0 },
        ];
        assert!(tokens.iter().zip(truth.iter()).map(|(&x, &y)| x == y).all(|x| x == true));
    }
}