
#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Timestamp,
    Character,
    End,
    Separator
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String
}
