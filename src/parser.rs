use crate::language::SyntaxNode;
use crate::syntax_kind::SyntaxKind;
use rowan::GreenNodeBuilder;

pub fn parse(input: &str) -> SyntaxNode {
    let tokens = crate::lexer::lex(input);
    let mut parser = Parser {
        tokens,
        pos: 0,
        builder: GreenNodeBuilder::new(),
    };
    parser.parse_source_file();
    let green = parser.builder.finish();
    SyntaxNode::new_root(green)
}

struct Parser {
    tokens: Vec<(SyntaxKind, String)>,
    pos: usize,
    builder: GreenNodeBuilder<'static>,
}

impl Parser {
    fn current(&self) -> Option<SyntaxKind> {
        self.peek_non_trivia()
    }

    fn peek_non_trivia(&self) -> Option<SyntaxKind> {
        let mut i = self.pos;
        while i < self.tokens.len() {
            let kind = self.tokens[i].0;
            if !kind.is_trivia() {
                return Some(kind);
            }
            i += 1;
        }
        None
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == Some(kind)
    }

    fn bump(&mut self) {
        if self.pos < self.tokens.len() {
            let (kind, ref text) = self.tokens[self.pos];
            self.builder.token(kind.into(), text);
            self.pos += 1;
        }
    }

    fn bump_trivia(&mut self) {
        while self.pos < self.tokens.len() && self.tokens[self.pos].0.is_trivia() {
            self.bump();
        }
    }

    fn expect(&mut self, kind: SyntaxKind) {
        self.bump_trivia();
        if self.pos < self.tokens.len() && self.tokens[self.pos].0 == kind {
            self.bump();
        }
    }

    fn parse_source_file(&mut self) {
        self.builder.start_node(SyntaxKind::SOURCE_FILE.into());
        self.bump_trivia();
        if self.at(SyntaxKind::IDENTIFICATION_KW) {
            self.parse_program_definition();
        }
        // Consume any trailing trivia
        while self.pos < self.tokens.len() {
            self.bump();
        }
        self.builder.finish_node();
    }

    fn parse_program_definition(&mut self) {
        self.builder
            .start_node(SyntaxKind::PROGRAM_DEFINITION.into());
        self.parse_identification_division();
        if self.at(SyntaxKind::PROCEDURE_KW) {
            self.parse_procedure_division();
        }
        self.builder.finish_node();
    }

    fn parse_identification_division(&mut self) {
        self.builder
            .start_node(SyntaxKind::IDENTIFICATION_DIVISION.into());
        // IDENTIFICATION
        self.expect(SyntaxKind::IDENTIFICATION_KW);
        // DIVISION
        self.bump_trivia();
        self.expect(SyntaxKind::DIVISION_KW);
        // .
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        // PROGRAM-ID clause
        self.bump_trivia();
        if self.at(SyntaxKind::PROGRAM_ID_KW) {
            self.parse_program_id_clause();
        }
        self.builder.finish_node();
    }

    fn parse_program_id_clause(&mut self) {
        self.builder
            .start_node(SyntaxKind::PROGRAM_ID_CLAUSE.into());
        // PROGRAM-ID
        self.expect(SyntaxKind::PROGRAM_ID_KW);
        // .
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        // <name>
        self.bump_trivia();
        self.expect(SyntaxKind::WORD);
        // .
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        self.builder.finish_node();
    }

    fn parse_procedure_division(&mut self) {
        self.builder
            .start_node(SyntaxKind::PROCEDURE_DIVISION.into());
        // PROCEDURE
        self.expect(SyntaxKind::PROCEDURE_KW);
        // DIVISION
        self.bump_trivia();
        self.expect(SyntaxKind::DIVISION_KW);
        // .
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        // Parse sentences
        self.bump_trivia();
        while self.pos < self.tokens.len() && self.current().is_some() {
            match self.current() {
                Some(SyntaxKind::DISPLAY_KW) | Some(SyntaxKind::STOP_KW) => {
                    self.parse_sentence();
                    self.bump_trivia();
                }
                _ => break,
            }
        }
        self.builder.finish_node();
    }

    fn parse_sentence(&mut self) {
        self.builder.start_node(SyntaxKind::SENTENCE.into());
        match self.current() {
            Some(SyntaxKind::DISPLAY_KW) => self.parse_display_statement(),
            Some(SyntaxKind::STOP_KW) => self.parse_stop_statement(),
            _ => {}
        }
        // Trailing DOT
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        self.builder.finish_node();
    }

    fn parse_display_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::DISPLAY_STATEMENT.into());
        // DISPLAY
        self.expect(SyntaxKind::DISPLAY_KW);
        // Operands (strings and words until DOT)
        self.bump_trivia();
        while self.pos < self.tokens.len() {
            match self.current() {
                Some(SyntaxKind::STRING_LITERAL) | Some(SyntaxKind::WORD) => {
                    self.expect(self.current().unwrap());
                    self.bump_trivia();
                }
                _ => break,
            }
        }
        self.builder.finish_node();
    }

    fn parse_stop_statement(&mut self) {
        self.builder.start_node(SyntaxKind::STOP_STATEMENT.into());
        // STOP
        self.expect(SyntaxKind::STOP_KW);
        // RUN
        self.bump_trivia();
        self.expect(SyntaxKind::RUN_KW);
        self.builder.finish_node();
    }
}
