use std::fmt;

struct LexicalError {
    message: String,
}

#[derive(Debug)]
pub enum Separator {
    Terminator,
    OpenParentheses,
    CloseParentheses,
    OpenQuotation,
    CloseQuotation,
    OpenCurlyBraces,
    CloseCurlyBraces,
}

const TERMINATOR: &str = "；";
const OPEN_PARENTHESES: &str = "（";
const CLOSED_PARENTHESES: &str = "）";
const OPEN_QUOTATION: &str = "「";
const CLOSED_QUOTATION: &str = "」";
const OPEN_CURLY_BRACES: &str = "｛";
const CLOSED_CURLY_BRACES: &str = "｝";

impl Separator {
    fn as_str(&self) -> &str {
        match self {
            Separator::Terminator => TERMINATOR,
            Separator::OpenParentheses => OPEN_PARENTHESES,
            Separator::CloseParentheses => CLOSED_PARENTHESES,
            Separator::OpenQuotation => OPEN_QUOTATION,
            Separator::CloseQuotation => CLOSED_QUOTATION,
            Separator::OpenCurlyBraces => OPEN_CURLY_BRACES,
            Separator::CloseCurlyBraces => CLOSED_CURLY_BRACES,
        }
    }
}

#[derive(Debug)]
pub enum ReservedWord {
    Function,
    Int,
    Str,
    If,
}

const FUNCTION: &str = "関数"; // かんすう
const INT: &str = "整数"; // なら
const IF: &str = "なら"; // せいすう
const STR: &str = "文字列"; // もじれつ

impl ReservedWord {
    fn as_str(&self) -> &str {
        match self {
            ReservedWord::Function => FUNCTION,
            ReservedWord::If => IF,
            ReservedWord::Int => INT,
            ReservedWord::Str => STR,
        }
    }
}

impl fmt::Display for ReservedWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matched = match self {
            ReservedWord::Function => FUNCTION,
            ReservedWord::If => IF,
            ReservedWord::Int => INT,
            ReservedWord::Str => STR,
        };
        write!(f, "{}", matched)
    }
}

impl TryFrom<&str> for ReservedWord {
    type Error = String;

    fn try_from(word: &str) -> Result<ReservedWord, Self::Error> {
        match word {
            FUNCTION => Ok(ReservedWord::Function),
            IF => Ok(ReservedWord::If),
            INT => Ok(ReservedWord::Int),
            STR => Ok(ReservedWord::Str),
            identifier => Err(String::from(identifier)),
        }
    }
}

impl TryFrom<String> for ReservedWord {
    type Error = String;

    fn try_from(word: String) -> Result<ReservedWord, Self::Error> {
        match word.as_str() {
            FUNCTION => Ok(ReservedWord::Function),
            IF => Ok(ReservedWord::If),
            INT => Ok(ReservedWord::Int),
            STR => Ok(ReservedWord::Str),
            identifier => Err(String::from(identifier)),
        }
    }
}
