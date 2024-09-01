use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Operator {
    Assignment,
    Sum,
    Subtraction,
    Multiplication,
    Division,
}

const ASSIGNMENT: &str = "＝";
const SUM: &str = "＋";
const SUBTRACTION: &str = "ー";
const MULTIPLICATION: &str = "＊";
const DIVISION: &str = "／";

impl Operator {
    fn as_str(&self) -> &str {
        match self {
            Operator::Assignment => ASSIGNMENT,
            Operator::Sum => SUM,
            Operator::Subtraction => SUBTRACTION,
            Operator::Multiplication => MULTIPLICATION,
            Operator::Division => DIVISION,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matched = match self {
            Operator::Assignment => ASSIGNMENT,
            Operator::Sum => SUM,
            Operator::Subtraction => SUBTRACTION,
            Operator::Multiplication => MULTIPLICATION,
            Operator::Division => DIVISION,
        };
        write!(f, "{}", matched)
    }
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(word: &str) -> Result<Operator, Self::Error> {
        match word {
            ASSIGNMENT => Ok(Operator::Assignment),
            SUM => Ok(Operator::Sum),
            SUBTRACTION => Ok(Operator::Subtraction),
            MULTIPLICATION => Ok(Operator::Multiplication),
            DIVISION => Ok(Operator::Division),
            identifier => Err(String::from(identifier)),
        }
    }
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

impl Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matched = match self {
            Separator::Terminator => TERMINATOR,
            Separator::OpenParentheses => OPEN_PARENTHESES,
            Separator::CloseParentheses => CLOSED_PARENTHESES,
            Separator::OpenQuotation => OPEN_QUOTATION,
            Separator::CloseQuotation => CLOSED_QUOTATION,
            Separator::OpenCurlyBraces => OPEN_CURLY_BRACES,
            Separator::CloseCurlyBraces => CLOSED_CURLY_BRACES,
        };
        write!(f, "{}", matched)
    }
}

impl TryFrom<&str> for Separator {
    type Error = String;

    fn try_from(word: &str) -> Result<Separator, Self::Error> {
        match word {
            TERMINATOR => Ok(Separator::Terminator),
            OPEN_PARENTHESES => Ok(Separator::OpenParentheses),
            CLOSED_PARENTHESES => Ok(Separator::CloseParentheses),
            OPEN_QUOTATION => Ok(Separator::OpenQuotation),
            CLOSED_QUOTATION => Ok(Separator::CloseQuotation),
            OPEN_CURLY_BRACES => Ok(Separator::OpenCurlyBraces),
            CLOSED_CURLY_BRACES => Ok(Separator::CloseCurlyBraces),
            identifier => Err(String::from(identifier)),
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

impl Display for ReservedWord {
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
