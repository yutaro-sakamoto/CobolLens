#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    WHITESPACE = 0,
    NEWLINE,
    DOT,
    STRING_LITERAL,
    WORD,
    ERROR,

    // Keywords
    IDENTIFICATION_KW,
    DIVISION_KW,
    PROGRAM_ID_KW,
    PROCEDURE_KW,
    DISPLAY_KW,
    STOP_KW,
    RUN_KW,

    // Nodes
    SOURCE_FILE,
    PROGRAM_DEFINITION,
    IDENTIFICATION_DIVISION,
    PROGRAM_ID_CLAUSE,
    PROCEDURE_DIVISION,
    SENTENCE,
    DISPLAY_STATEMENT,
    STOP_STATEMENT,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WHITESPACE | SyntaxKind::NEWLINE)
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
