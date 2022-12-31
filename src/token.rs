pub enum TokenKind {
    // Identifiers
    Identifier(String),

    // Literals
    IntLiteral(i32),
    FloatLiteral(f32),
    CharLiteral(String), // make it a string so it prints a bit nicer.
    StringLiteral(String),

    // Braces
    // paren = ( ), brace = { }, bracket = [ ]
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Operators
    PlusOp,           // +
    MinusOp,          // -
    StarOp,           // *
    DivOp,            // /
    ModOp,            // %
    IncrementOp,      // ++
    DecrementOp,      // --
    EqOp,             // =
    AssignOp,         // :=
    BitAndOp,         // &
    BitOrOp,          // |
    BitXorOp,         // ^
    BitClearOp,       // &^ negates the bits
    BitNotOp,         // ~
    PlusEqOp,         // +=
    MinusEqOp,        // -=
    StarEqOp,         // *=
    DivEqOp,          // /=
    ModEqOp,          // %=
    BitAndEqOp,       // &=
    BitOrEqOp,        // |=
    BitXorEqOp,       // ^=
    BitClearEqOp,     // &^=
    LogicAndOp,       // &&
    LogicOrOp,        // ||
    LogicNotOp,       // !
    LogicEqOp,        // ==
    LShiftOp,         // <<
    LShiftEqOp,       // <<=
    RShiftOp,         // >>
    RShiftEqOp,       // >>=
    LogicNotEqOp,     // !=
    LogicLessOp,      // <
    LogicGreaterOp,   // >
    LogicLessEqOp,    // <=
    LogicGreaterEqOp, // >=
    DirectionOp,      // <-

    // Punctuation
    Colon,            // :
    Semicolon,        // ;
    Dot,              // .
    Comma,            // ,
    Elipses,          // ...
    DoubleQuote,      // "
    SingleQuote,      // '

    // Keywords
    BreakKeyword,
    DefaultKeyword,
    FuncKeyword,
    InterfaceKeyword,
    SelectKeyword,
    CaseKeyword,
    DeferKeyword,
    GoKeyword,
    MapKeyword,
    StructKeyword,
    ChanKeyword,
    ElseKeyword,
    GotoKeyword,
    PackageKeyword,
    SwitchKeyword,
    ConstKeyword,
    FallThroughKeyword,
    IfKeyword,
    RangeKeyword,
    TypeKeyword,
    ContinueKeyword,
    ForKeyword,
    ImportKeyword,
    ReturnKeyword,
    VarKeyword,

    // End of file.
    EOF,
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            Self::Identifier(name) => name.clone(),
            Self::IntLiteral(number) => format!("{}", number),
            Self::FloatLiteral(number) => format!("{}", number),
            Self::CharLiteral(character) => character.to_string(),
            Self::StringLiteral(str) => str.clone(),
            _ => match self {
                Self::LeftParen => "(",
                Self::RightParen => ")",
                Self::LeftBrace => "{",
                Self::RightBrace => "}",
                Self::LeftBracket => "[",
                Self::RightBracket => "]",
                Self::PlusOp => "+",
                Self::MinusOp => "-",
                Self::StarOp => "*",
                Self::DivOp => "/",
                Self::ModOp => "%",
                Self::IncrementOp => "++",
                Self::DecrementOp => "--",
                Self::EqOp => "=",
                Self::AssignOp => ":=",
                Self::BitAndOp => "&",
                Self::BitOrOp => "|",
                Self::BitXorOp => "^",
                Self::BitClearOp => "&^",
                Self::BitNotOp => "~",
                Self::PlusEqOp => "+=",
                Self::MinusEqOp => "-=",
                Self::StarEqOp => "*=",
                Self::DivEqOp => "/=",
                Self::ModEqOp => "%=",
                Self::BitAndEqOp => "&=",
                Self::BitOrEqOp => "|=",
                Self::BitXorEqOp => "^=",
                Self::BitClearEqOp => "&^=",
                Self::LogicAndOp => "&&",
                Self::LogicOrOp => "||",
                Self::LogicNotOp => "!",
                Self::LogicEqOp => "==",
                Self::LShiftOp => "<<",
                Self::LShiftEqOp => "<<=",
                Self::RShiftOp => ">>",
                Self::RShiftEqOp => ">>=",
                Self::LogicNotEqOp => "!=",
                Self::LogicLessOp => "<",
                Self::LogicGreaterOp => ">",
                Self::LogicLessEqOp => "<=",
                Self::LogicGreaterEqOp => ">=",
                Self::DirectionOp => "<-",
                Self::Colon => ":",
                Self::Semicolon => ";",
                Self::Dot => ".",
                Self::Comma => ",",
                Self::Elipses => "...",
                Self::DoubleQuote => "\"",
                Self::SingleQuote => "\'",
                Self::BreakKeyword => "break",
                Self::DefaultKeyword => "default",
                Self::FuncKeyword => "func",
                Self::InterfaceKeyword => "interface",
                Self::SelectKeyword => "select",
                Self::CaseKeyword => "case",
                Self::DeferKeyword => "defer",
                Self::GoKeyword => "go",
                Self::MapKeyword => "map",
                Self::StructKeyword => "struct",
                Self::ChanKeyword => "chan",
                Self::ElseKeyword => "else",
                Self::GotoKeyword => "goto",
                Self::PackageKeyword => "package",
                Self::SwitchKeyword => "switch",
                Self::ConstKeyword => "const",
                Self::FallThroughKeyword => "fallthrough",
                Self::IfKeyword => "if",
                Self::RangeKeyword => "range",
                Self::TypeKeyword => "type",
                Self::ContinueKeyword => "continue",
                Self::ForKeyword => "for",
                Self::ImportKeyword => "import",
                Self::ReturnKeyword => "return",
                Self::VarKeyword => "var",
                Self::EOF => "EOF",
                _ => unreachable!(),
            }
            .to_string(),
        }
    }
}