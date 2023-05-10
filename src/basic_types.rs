#[derive(Clone, Copy)]
pub enum TokenType {
    Opertaion,
    KeyWord,
    Comparative,
}

#[derive(Clone, Copy)]
pub enum OpertaionType {
    Push,
    Dump,
    Dup,
    Dup2,
    Swap,
    Swap2,
    Blank,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Shr,
    Shl,
    Bor,
    Band,
    Xor,
    Pop,
    Put,
    PutSpace,
    PutUnicode,
}

#[derive(Clone, Copy)]
pub enum ComparativeType {
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equal,
    And,
    Or,
    Not,
}


#[derive(Clone, Copy)]
pub enum KeyWordType {
    Do,
    While,
    End,
    Else,
}
