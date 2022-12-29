pub enum TokenKind {
    // Identifiers
    Identifier(String),

    // Literals
    IntLiteral(i32),
    FloatLiteral(f32),
    CharLiteral(char),
    StringLiteral(String),
    
    // Braces
    // paren = ( ), brace = { }, bracket = [ ]
    LeftParen,          // (
    RightParen,         // )
    LeftBrace,          // {
    RightBrace,         // }
    LeftBracket,        // [
    RightBracket,       // ]

    // Operators
    PlusOp,             // +
    MinusOp,            // - 
    StarOp,             // *
    DivOp,              // /
    ModOp,              // %
    IncrementOp,        // ++
    DecrementOp,        // --
    EqOp,               // =
    AssignOp,           // :=
    BitAndOp,           // &
    BitOrOp,            // |
    BitXorOp,           // ^
    BitClearOp,         // &^ negates the bits
    BitNotOp,           // ~
    PlusEqOp,           // +=
    MinusEqOp,          // -=
    StarEqOp,           // *=
    DivEqOp,            // /=
    ModEqOp,            // %=
    BitAndEqOp,         // &=
    BitOrEqOp,          // |=
    BitXorEqOp,         // ^=
    BitClearEqOp,       // &^=
    LogicAndOp,         // &&
    LogicOrOp,          // ||
    LogicNotOp,         // !
    LogicEqOp,          // ==
    LogicLShiftOp,      // <<
    LogicLShiftEqOp,    // <<=
    LogicRShiftOp,      // >>
    LogicRShiftEqOp,    // >>=
    LogicNotEqOp,       // !=
    LogicLessOp,        // <
    LogicGreaterOp,     // >
    LogicLessEqOp,      // <=
    LogicGreaterEqOp,   // >=
    DirectionOp,        // <-

    // Punctuation
    Colon,              // ,
    Semicolon,          // ;
    Dot,                // .
    Comma,              // ,
    Elipses,            // ...   

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

    EOF,
}
