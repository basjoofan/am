use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    Illegal, // illegal token
    Eof,     // end of file

    // ident + literal
    Ident,    // add, foobar, x, y, ...
    Integer,  // 56789
    Float,    // 3.14159265358979323846264338327950288
    True,     // true
    False,    // false
    String,   // "foobar"
    Template, // `GET http://example.com`

    // operator
    Assign,  // =
    Bang,    // !
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Bx,      // ^
    Bo,      // |
    Ba,      // &
    Ll,      // <<
    Gg,      // >>
    Lo,      // ||
    La,      // &&
    Lt,      // <
    Gt,      // >
    Le,      // <=
    Ge,      // >=
    Eq,      // ==
    Ne,      // !=

    // delimiter
    Comma, // ,
    Semi,  // ;
    Colon, // :
    Dot,   // .

    // couple
    Lp, // (
    Rp, // )
    Lb, // {
    Rb, // }
    Ls, // [
    Rs, // ]

    // keyword
    Fn,     // fn
    Rq,     // rq
    Let,    // let
    If,     // if
    Else,   // else
    Return, // return
    Test,   // test
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: Kind, literal: String) -> Token {
        Token { kind, literal }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            Kind::Lo => 1,       // a || b
            Kind::La => 2,       // a && b
            Kind::Bo => 3,       // a | b
            Kind::Bx => 4,       // a ^ b
            Kind::Ba => 5,       // a & b
            Kind::Eq => 6,       // a == b
            Kind::Ne => 6,       // a != b
            Kind::Lt => 7,       // a < b
            Kind::Gt => 7,       // a > b
            Kind::Le => 7,       // a <= b
            Kind::Ge => 7,       // a >= b
            Kind::Ll => 8,       // a << b
            Kind::Gg => 8,       // a >> b
            Kind::Plus => 9,     // a + b
            Kind::Minus => 9,    // a - b
            Kind::Star => 10,    // a * b
            Kind::Slash => 10,   // a / b
            Kind::Percent => 10, // a / b
            // Kind::Minus => 11,  -x unary minus + 2
            Kind::Bang => 11, // !x
            Kind::Lp => 12,   // function()
            Kind::Ls => 13,   // array[index]
            Kind::Dot => 13,  // object.field
            _ => 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.literal)
    }
}
