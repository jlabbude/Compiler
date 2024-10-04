use reserved_proc_macro::Reserved;
use strum_macros::EnumIter;

#[derive(Reserved, Debug)]
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

#[derive(Reserved, Debug, EnumIter)]
pub enum Operator {
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

#[derive(Reserved, Debug, PartialEq, Eq, EnumIter)]
pub enum Separator {
    #[word("//")]
    InlineComment,
    #[word(";")]
    Terminator,
    #[word("(")]
    OpenParentheses,
    #[word(")")]
    CloseParentheses,
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
