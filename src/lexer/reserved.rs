use reserved_proc_macro::Reserved;
use strum_macros::EnumIter;

#[derive(Reserved, Debug, PartialEq, Clone)]
pub enum ReservedWord {
    #[word("func")]
    Function,
    #[word("return")]
    Return,
    #[word("if")]
    If,
    #[word("else")]
    Else,
    #[word("match")]
    Match,
    #[word("case")]
    Case,
    #[word("default")]
    Default,
    #[word("for")]
    For,
    #[word("while")]
    While,
    #[word("let")]
    Let,
    #[word("const")]
    Constant,
    #[word("int")]
    Int,
    #[word("long")]
    Long,
    #[word("float")]
    Float,
    #[word("double")]
    Double,
    #[word("string")]
    Str,
    #[word("character")]
    Char,
    #[word("boolean")]
    Bool,
    #[word("void")]
    Void,
    #[word("struct")]
    Struct,
    #[word("enumeration")]
    Enum,
}

#[derive(Reserved, Debug, EnumIter, PartialEq, Clone)]
pub enum Operator {
    #[word("+=")]
    Increment,
    #[word(">=")]
    GreaterThanOrEqual,
    #[word("<=")]
    LessThanOrEqual,
    #[word("==")]
    Equality,
    #[word("!=")]
    Inequality,
    #[word("!")]
    Negation,
    #[word("=")]
    Assignment,
    #[word("+")]
    Sum,
    #[word("-")]
    Subtraction,
    #[word("*")]
    Multiplication,
    #[word("/")]
    Division,
    #[word(">")]
    GreaterThan,
    #[word("<")]
    LessThan,
}

#[derive(Reserved, Debug, PartialEq, Eq, EnumIter, Clone)]
pub enum Separator {
    #[word("//")]
    InlineComment,
    #[word("/*")]
    CommentBlockOpen,
    #[word("*/")]
    CommentBlockClose,
    #[word(";")]
    Terminator,
    #[word("(")]
    OpenParenthesis,
    #[word(")")]
    CloseParenthesis,
    #[word("\"")]
    StringQuotation,
    #[word("'")]
    CharQuotation,
    #[word("{")]
    OpenCurlyBraces,
    #[word("}")]
    CloseCurlyBraces,
    #[word("[")]
    OpenBrackets,
    #[word("]")]
    CloseBrackets,
    #[word(",")]
    Comma,
    #[word(".")]
    Dot,
    #[word("\n")]
    NewLine,
    #[word(" ")]
    WhiteSpace,
}
