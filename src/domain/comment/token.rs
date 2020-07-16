#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenKind {
    Timestamp,
    Character,
    End,
    Separator,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}
