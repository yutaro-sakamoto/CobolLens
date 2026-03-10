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
    NUMERIC_LITERAL,

    // Fixed-format trivia tokens
    SEQUENCE_NUMBER,
    INDICATOR,
    COMMENT,
    IDENTIFICATION_AREA,

    // Punctuation / operators
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    EQUALS_SIGN,
    PLUS,
    MINUS,
    STAR,
    SLASH,

    // Keywords
    IDENTIFICATION_KW,
    DIVISION_KW,
    PROGRAM_ID_KW,
    PROCEDURE_KW,
    DISPLAY_KW,
    STOP_KW,
    RUN_KW,
    ENVIRONMENT_KW,
    CONFIGURATION_KW,
    SOURCE_COMPUTER_KW,
    OBJECT_COMPUTER_KW,
    INPUT_OUTPUT_KW,
    FILE_CONTROL_KW,
    DATA_KW,
    FILE_KW,
    FD_KW,
    WORKING_STORAGE_KW,
    SECTION_KW,
    SELECT_KW,
    ASSIGN_KW,
    TO_KW,
    PICTURE_KW,
    PIC_KW,
    VALUE_KW,
    REDEFINES_KW,
    COMPUTATIONAL_KW,
    COMP_KW,
    FILLER_KW,
    IS_KW,
    ZERO_KW,
    ZEROS_KW,
    SPACE_KW,
    SPACES_KW,
    OPEN_KW,
    OUTPUT_KW,
    INPUT_KW,
    CLOSE_KW,
    MOVE_KW,
    PERFORM_KW,
    THRU_KW,
    THROUGH_KW,
    TIMES_KW,
    GO_KW,
    IF_KW,
    ELSE_KW,
    NOT_KW,
    EQUAL_KW,
    GREATER_KW,
    LESS_KW,
    THAN_KW,
    ADD_KW,
    MULTIPLY_KW,
    BY_KW,
    ROUNDED_KW,
    ON_KW,
    SIZE_KW,
    ERROR_KW,
    END_MULTIPLY_KW,
    WRITE_KW,
    AFTER_KW,
    ADVANCING_KW,
    PAGE_KW,
    LINES_KW,
    LINE_KW,
    EXIT_KW,
    RECORD_KW,
    OPTIONAL_KW,
    DELETE_KW,

    // Nodes
    SOURCE_FILE,
    PROGRAM_DEFINITION,
    IDENTIFICATION_DIVISION,
    PROGRAM_ID_CLAUSE,
    PROCEDURE_DIVISION,
    SENTENCE,
    DISPLAY_STATEMENT,
    STOP_STATEMENT,
    ENVIRONMENT_DIVISION,
    CONFIGURATION_SECTION,
    SOURCE_COMPUTER_PARAGRAPH,
    OBJECT_COMPUTER_PARAGRAPH,
    INPUT_OUTPUT_SECTION,
    FILE_CONTROL_PARAGRAPH,
    SELECT_CLAUSE,
    DATA_DIVISION,
    FILE_SECTION,
    FD_ENTRY,
    WORKING_STORAGE_SECTION,
    DATA_DESCRIPTION_ENTRY,
    PICTURE_CLAUSE,
    VALUE_CLAUSE,
    REDEFINES_CLAUSE,
    USAGE_CLAUSE,
    SECTION_HEADER,
    PARAGRAPH,
    PARAGRAPH_NAME,
    OPEN_STATEMENT,
    CLOSE_STATEMENT,
    MOVE_STATEMENT,
    PERFORM_STATEMENT,
    GO_TO_STATEMENT,
    IF_STATEMENT,
    ELSE_CLAUSE,
    ADD_STATEMENT,
    MULTIPLY_STATEMENT,
    ON_SIZE_ERROR_CLAUSE,
    NOT_ON_SIZE_ERROR_CLAUSE,
    WRITE_STATEMENT,
    ADVANCING_CLAUSE,
    EXIT_STATEMENT,
    CONDITION,
    // Must be last
    __LAST,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE
                | SyntaxKind::NEWLINE
                | SyntaxKind::SEQUENCE_NUMBER
                | SyntaxKind::INDICATOR
                | SyntaxKind::COMMENT
                | SyntaxKind::IDENTIFICATION_AREA
                | SyntaxKind::COMMA
                | SyntaxKind::SEMICOLON
        )
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
