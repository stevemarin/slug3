
use crate::{token::{Token, TokenType, Number, Operator, Keyword}, chunk::Op};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Exponent,
    Unary,
    Call,
    Primary
}

impl From<TokenType> for ParseRule {
    fn from(val: TokenType) -> Self {
        match val {
            TokenType::Operator(Operator::LeftParen) => ParseRule { prefix: Some(grouping), infix: Some(call), precedence: Precedence::Call },
            TokenType::Operator(Operator::RightParen) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },

            TokenType::Operator(Operator::Plus) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Term },
            TokenType::Operator(Operator::Minus) => ParseRule { prefix: Some(unary), infix: Some(binary), precedence: Precedence::Term },
            TokenType::Operator(Operator::Star) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
            TokenType::Operator(Operator::Slash) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
            TokenType::Operator(Operator::SlashSlash) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
            TokenType::Operator(Operator::StarStar) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Exponent },
            
            TokenType::Operator(Operator::Greater) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(Operator::GreaterEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(Operator::Less) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(Operator::LessEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(Operator::EqualEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Equality },
            TokenType::Operator(Operator::NotEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Equality },

            TokenType::Operator(Operator::Pound) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Operator(Operator::Equal) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },

            TokenType::Number(Number::Integer) => ParseRule { prefix: Some(integer), infix: None, precedence: Precedence::None },
            TokenType::Number(Number::Float) => ParseRule { prefix: Some(float), infix: None, precedence: Precedence::None },
            TokenType::Number(Number::Complex) => ParseRule { prefix: Some(complex), infix: None, precedence: Precedence::None },

            TokenType::Keyword(Keyword::Assert) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::If) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::In) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::Not) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::Else) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::Elif) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::While) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::For) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(Keyword::Class) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },

            TokenType::Identifier => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Eof => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
        }
    }
}

type ParseFn = fn(&mut Parser, bool) -> ();

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence
}

struct Local {
    name: Token,
    depth: u8,
    captured: bool
}

enum FuncType {
    Function,
    Initializer,
    Method,
    Script,
}

pub struct Parser {
    current_index: usize,
    tokens: Vec<Token>,
    had_error: bool,
    panic_mode: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { current_index: 0, tokens, had_error: false, panic_mode: false }
    }

    #[inline]
    fn current(&self) -> Token {
        *self.tokens.get(self.current_index).unwrap()
    }

    #[inline]
    fn previous(&self) -> Token {
        *self.tokens.get(self.current_index - 1).unwrap()
    }

    #[inline]
    fn check(&self, tokentype: TokenType) -> bool {
        self.tokens.get(self.current_index).unwrap().tokentype == tokentype
    }

    #[inline]
    fn match_tokentype(&mut self, tokentype: TokenType) -> bool {
        let m = self.check(tokentype);
        if m {
            self.current_index += 1;
        }
        m
    }

    #[inline]
    fn consume(&mut self, tokentype: TokenType, msg: String) {
        if !self.match_tokentype(tokentype) {
            panic!("{:?}", msg)
        }
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.current_index += 1;

        let assignable = precedence <= Precedence::Assignment;

        let prefix = Into::<ParseRule>::into(self.previous().tokentype).prefix;
        match prefix {
            Some(parsefn) => parsefn(self, assignable),
            None => panic!("prefix is None")
        }

        while precedence <= Into::<ParseRule>::into(self.current().tokentype).precedence {
            self.current_index += 1;

            let infix = Into::<ParseRule>::into(self.previous().tokentype).infix;
            match infix {
                Some(parsefn) => parsefn(self, assignable),
                None => panic!("infix is None")
            }
        }

        if assignable && self.match_tokentype(TokenType::Operator(Operator::Equal)) {
            panic!("invalid assignment target")
        }

    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn expression_statement(&mut self) {
        self.expression()
    }

    fn statement(&mut self) {
        if self.match_tokentype(TokenType::Keyword(Keyword::Assert)) {
            self.assert();
        } else {
            self.expression_statement()
        }
    }

    fn declaration(&mut self) {
        self.statement()
    }

    fn assert(&mut self) {
        self.expression();
        emit_byte(Op::Assert, self.previous().line);
    }

}

fn emit_byte(op: Op, line: usize) {
    println!("emitting bytes");
    op;
    line;
}

fn unary(_parser: &mut Parser, _: bool) {

}

fn binary(_parser: &mut Parser, _: bool) {

}

fn grouping(_parser: &mut Parser, _: bool) {

}

fn call(_parser: &mut Parser, _: bool) {

}

fn integer(_parser: &mut Parser, _: bool) {

}

fn float(_parser: &mut Parser, _: bool) {

}

fn complex(_parser: &mut Parser, _: bool) {

}
