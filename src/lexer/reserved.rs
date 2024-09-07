use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum ReservedWord {
    Function,
    Return,
    If,
    Else,
    Int,
    Str,
    Bool,
    Void,
}

const FUNCTION: &str = "関数"; // かんすう
const RETURN: &str = "返す"; // かえす
const IF: &str = "もし";
const ELSE: &str = "それとも";
const INT: &str = "整数"; // せいすう
const STR: &str = "文字列"; // もじれつ
const BOOL: &str = "真偽値"; // しんぎち
const VOID: &str = "無"; // む

impl Display for ReservedWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matched = match self {
            ReservedWord::Function => FUNCTION,
            ReservedWord::Return => RETURN,
            ReservedWord::If => IF,
            ReservedWord::Else => ELSE,
            ReservedWord::Int => INT,
            ReservedWord::Str => STR,
            ReservedWord::Bool => BOOL,
            ReservedWord::Void => VOID,
        };
        write!(f, "{}", matched)
    }
}

impl TryFrom<&str> for ReservedWord {
    type Error = String;

    fn try_from(word: &str) -> Result<ReservedWord, Self::Error> {
        match word {
            FUNCTION => Ok(ReservedWord::Function),
            RETURN => Ok(ReservedWord::Return),
            IF => Ok(ReservedWord::If),
            ELSE => Ok(ReservedWord::Else),
            INT => Ok(ReservedWord::Int),
            STR => Ok(ReservedWord::Str),
            BOOL => Ok(ReservedWord::Bool),
            VOID => Ok(ReservedWord::Void),
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
            BOOL => Ok(ReservedWord::Bool),
            identifier => Err(String::from(identifier)),
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Assignment,
    Sum,
    Subtraction,
    Multiplication,
    Division,
    Equality,
    Inequality,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

const ASSIGNMENT: &str = "＝";
const SUM: &str = "＋";
const SUBTRACTION: &str = "ー";
const MULTIPLICATION: &str = "＊";
const DIVISION: &str = "／";
const EQUALITY: &str = "＝＝";
const INEQUALITY: &str = "！＝";
const GREATER_THAN: &str = "＞";
const LESS_THAN: &str = "＜";
const GREATER_THAN_OR_EQUAL: &str = "＞＝";
const LESS_THAN_OR_EQUAL: &str = "＜＝";

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matched = match self {
            Operator::Assignment => ASSIGNMENT,
            Operator::Sum => SUM,
            Operator::Subtraction => SUBTRACTION,
            Operator::Multiplication => MULTIPLICATION,
            Operator::Division => DIVISION,
            Operator::Equality => EQUALITY,
            Operator::Inequality => INEQUALITY,
            Operator::GreaterThan => GREATER_THAN,
            Operator::LessThan => LESS_THAN,
            Operator::GreaterThanOrEqual => GREATER_THAN_OR_EQUAL,
            Operator::LessThanOrEqual => LESS_THAN_OR_EQUAL,
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
            EQUALITY => Ok(Operator::Equality),
            INEQUALITY => Ok(Operator::Inequality),
            GREATER_THAN => Ok(Operator::GreaterThan),
            LESS_THAN => Ok(Operator::LessThan),
            GREATER_THAN_OR_EQUAL => Ok(Operator::GreaterThanOrEqual),
            LESS_THAN_OR_EQUAL => Ok(Operator::LessThanOrEqual),
            other_token => Err(String::from(other_token)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Separator {
    Terminator,
    OpenParentheses,
    CloseParentheses,
    OpenQuotation,
    CloseQuotation,
    OpenCurlyBraces,
    CloseCurlyBraces,
    Comma,
    Dot,
    NewLine,
    WhiteSpace,
}

const TERMINATOR: &str = "；";
const OPEN_PARENTHESES: &str = "（";
const CLOSED_PARENTHESES: &str = "）";
const OPEN_QUOTATION: &str = "「";
const CLOSED_QUOTATION: &str = "」";
const OPEN_CURLY_BRACES: &str = "｛";
const CLOSED_CURLY_BRACES: &str = "｝";
const COMMA: &str = "、";
const DOT: &str = "。";
const NEW_LINE: &str = "\n";
const REGULAR_SPACE: &str = " ";
const JP_SPACE: &str = "　";

impl Separator {
    pub fn as_str(&self) -> &str {
        match self {
            Separator::Terminator => TERMINATOR,
            Separator::OpenParentheses => OPEN_PARENTHESES,
            Separator::CloseParentheses => CLOSED_PARENTHESES,
            Separator::OpenQuotation => OPEN_QUOTATION,
            Separator::CloseQuotation => CLOSED_QUOTATION,
            Separator::OpenCurlyBraces => OPEN_CURLY_BRACES,
            Separator::CloseCurlyBraces => CLOSED_CURLY_BRACES,
            Separator::Comma => COMMA,
            Separator::Dot => DOT,
            Separator::NewLine => NEW_LINE,
            Separator::WhiteSpace => REGULAR_SPACE,
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
            Separator::Comma => COMMA,
            Separator::Dot => DOT,
            Separator::NewLine => NEW_LINE,
            Separator::WhiteSpace => JP_SPACE,
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
            COMMA => Ok(Separator::Comma),
            DOT => Ok(Separator::Dot),
            REGULAR_SPACE | JP_SPACE => Ok(Separator::WhiteSpace),
            NEW_LINE => Ok(Separator::NewLine),
            other_token => Err(String::from(other_token)),
        }
    }
}
