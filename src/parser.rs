
use crate::{token::{Token, TokenType, OperatorType, KeywordType}, chunk::Op};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
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

impl Into<ParseRule> for TokenType {
    fn into(self) -> ParseRule {
        match self {
            TokenType::Operator(OperatorType::LeftParen) => ParseRule { prefix: Some(grouping), infix: Some(call), precedence: Precedence::Call },
            TokenType::Operator(OperatorType::RightParen) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },

            TokenType::Operator(OperatorType::Plus) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Term },
            TokenType::Operator(OperatorType::Minus) => ParseRule { prefix: Some(unary), infix: Some(binary), precedence: Precedence::Term },
            TokenType::Operator(OperatorType::Star) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
            TokenType::Operator(OperatorType::Slash) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
            TokenType::Operator(OperatorType::SlashSlash) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
            TokenType::Operator(OperatorType::StarStar) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Exponent },
            
            TokenType::Operator(OperatorType::Greater) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(OperatorType::GreaterEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(OperatorType::Less) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(OperatorType::LessEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Comparison },
            TokenType::Operator(OperatorType::EqualEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Equality },
            TokenType::Operator(OperatorType::NotEqual) => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Equality },

            TokenType::Operator(OperatorType::Pound) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Operator(OperatorType::Equal) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },

            TokenType::Keyword(KeywordType::Integer) => ParseRule { prefix: Some(integer), infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::Float) => ParseRule { prefix: Some(float), infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::Complex) => ParseRule { prefix: Some(complex), infix: None, precedence: Precedence::None },

            TokenType::Keyword(KeywordType::Assert) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::If) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::In) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::Not) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::Else) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::Elif) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::While) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::For) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Keyword(KeywordType::Class) => ParseRule { prefix: None, infix: None, precedence: Precedence::None },

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

    fn parse_precedence(&mut self, precedence: Precedence) {
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

        if assignable && self.match_tokentype(TokenType::Operator(OperatorType::Equal)) {
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
        if self.match_tokentype(TokenType::Keyword(KeywordType::Assert)) {
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
    op;
    line;
}

fn unary(parser: &mut Parser, _: bool) {

}

fn binary(parser: &mut Parser, _: bool) {

}

fn grouping(parser: &mut Parser, _: bool) {

}

fn call(parser: &mut Parser, _: bool) {

}

fn integer(parser: &mut Parser, _: bool) {

}

fn float(parser: &mut Parser, _: bool) {

}

fn complex(parser: &mut Parser, _: bool) {

}