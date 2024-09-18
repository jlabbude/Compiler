use reserved_proc_macro::Reserved;
use strum_macros::EnumIter;

#[derive(Reserved, Debug)]
pub enum ReservedWord {
    #[word("関数")]
    Function,
    #[word("返す")]
    Return,
    #[word("もし")]
    If,
    #[word("それとも")]
    Else,
    #[word("整数")]
    Int,
    #[word("浮動小数点数")]
    Float,
    #[word("倍精度浮動小数点数")]
    Double,
    #[word("文字列")]
    Str,
    #[word("字")]
    Char,
    #[word("真偽値")]
    Bool,
    #[word("無")]
    Void,
    #[word("複合")]
    Struct,
    #[word("列挙")]
    Enum,
}

#[derive(Reserved, Debug, EnumIter)]
pub enum Operator {
    #[word("＞＝")]
    GreaterThanOrEqual,
    #[word("＜＝")]
    LessThanOrEqual,
    #[word("＝＝")]
    Equality,
    #[word("！＝")]
    Inequality,
    #[word("！")]
    Negation,
    #[word("＝")]
    Assignment,
    #[word("＋")]
    Sum,
    #[word("ー")]
    Subtraction,
    #[word("＊")]
    Multiplication,
    #[word("／")]
    Division,
    #[word("＞")]
    GreaterThan,
    #[word("＜")]
    LessThan,
}

#[derive(Reserved, Debug, PartialEq, Eq, EnumIter)]
pub enum Separator {
    #[word("；")]
    Terminator,
    #[word("（")]
    OpenParentheses,
    #[word("）")]
    CloseParentheses,
    #[word("「")]
    OpenQuotation,
    #[word("」")]
    CloseQuotation,
    #[word("｛")]
    OpenCurlyBraces,
    #[word("｝")]
    CloseCurlyBraces,
    #[word("〚")]
    OpenBrackets,
    #[word("〛")]
    CloseBrackets,
    #[word("、")]
    Comma,
    #[word("。")]
    Dot,
    #[word("\n")]
    NewLine,
    #[word("　", " ")]
    WhiteSpace,
}
