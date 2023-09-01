/// # tokenise.rs Module
///
/// The tokenisation module for the KiraCode compiler, version 'test0'.
// ---

// Imports
use std::{fs::File, io::Read};

/// ## TokenType Enum
pub enum TokenType {
    Expr,
    ExprLtrl,
    ExprLtrlNum,
    ExprLtrlCha,
    ExprLtrlStr,
    ExprVar,
    ExprFun,
    ExprBlock,
    ExprConst,
    ExprAlias,
    ExprStruc,
    ExprEnum,
    ExprBltn,
    ExprBltnExit,
    Stmt,
    StmtLet,
    StmtFun,
    StmtConst,
    StmtAlias,
    StmtStruc,
    StmtEnum,
    StmtBodyStruc,
    StmtBodyEnum,
    Type,
    Oper,
    OperOpen,
    OperOpenBrace,
    OperOpenBrack,
    OperOpenParen,
    OperClose,
    OperCloseBrace,
    OperCloseBrack,
    OperCloseParen,
    OperMths,
    OperMthsDiv,
    OperMthsMul,
    OperMthsAdd,
    OperMthsSub,
    OperMthsMod,
    OperAsgn,
    OperAsgnEq,
    OperAsgnDivEq,
    OperAsgnMulEq,
    OperAsgnAddEq,
    OperAsgnSubEq,
    OperLogcEq,
    OperLogcNeq,
    OperLogcLt,
    OperLogcLteq,
    OperLogcGt,
    OperLogcGteq,
    OperLogcAnd,
    OperLogcOr,
    OperLogcNot,
}

/// ## Token Struct
pub struct Token {
    src: String,
    token_type: TokenType,
}

impl Token {
    pub fn new(src: String, token_type: TokenType) -> Self {
        Self {
            src: src,
            token_type: token_type,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
}

pub struct Tokeniser {
    src: String,
}

impl Tokeniser {
    pub fn new(src: &mut File) -> Self {
        let mut src_string: String = String::new();

        src.read_to_string(&mut src_string).unwrap();

        Self { src: src_string }
    }

    pub fn peek(&self, step: usize) -> Option<Token> {
        panic!("Unimplemented method: peek()");
    }

    pub fn consume(&self) -> Option<Token> {
        panic!("Unimplemented method: peek()");
    }
}
