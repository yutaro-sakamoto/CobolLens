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

    fn peek_non_trivia_text(&self) -> Option<&str> {
        let mut i = self.pos;
        while i < self.tokens.len() {
            let kind = self.tokens[i].0;
            if !kind.is_trivia() {
                return Some(&self.tokens[i].1);
            }
            i += 1;
        }
        None
    }

    /// Return the kind of the nth non-trivia token (0-based) from the current position.
    fn peek_nth_non_trivia(&self, n: usize) -> Option<SyntaxKind> {
        let mut count = 0;
        let mut i = self.pos;
        while i < self.tokens.len() {
            let kind = self.tokens[i].0;
            if !kind.is_trivia() {
                if count == n {
                    return Some(kind);
                }
                count += 1;
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

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        self.bump_trivia();
        if self.pos < self.tokens.len() && self.tokens[self.pos].0 == kind {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Consume any single non-trivia token (word, keyword, literal, operator, etc.)
    fn bump_any(&mut self) {
        self.bump_trivia();
        if self.pos < self.tokens.len() {
            self.bump();
        }
    }

    /// Check if the current non-trivia token is a level number (01-49, 66, 77, 88)
    fn at_level_number(&self) -> bool {
        if self.current() != Some(SyntaxKind::NUMERIC_LITERAL) {
            return false;
        }
        matches!(
            self.peek_non_trivia_text()
                .and_then(|s| s.parse::<u32>().ok()),
            Some(1..=49 | 66 | 77 | 88)
        )
    }

    /// Check if we're at a statement-starting keyword
    fn at_statement_start(&self) -> bool {
        matches!(
            self.current(),
            Some(SyntaxKind::DISPLAY_KW)
                | Some(SyntaxKind::STOP_KW)
                | Some(SyntaxKind::MOVE_KW)
                | Some(SyntaxKind::PERFORM_KW)
                | Some(SyntaxKind::GO_KW)
                | Some(SyntaxKind::IF_KW)
                | Some(SyntaxKind::ADD_KW)
                | Some(SyntaxKind::MULTIPLY_KW)
                | Some(SyntaxKind::WRITE_KW)
                | Some(SyntaxKind::OPEN_KW)
                | Some(SyntaxKind::CLOSE_KW)
                | Some(SyntaxKind::EXIT_KW)
        )
    }

    /// Is the current position at a section header (WORD SECTION)?
    fn at_section_header(&self) -> bool {
        let first = self.peek_nth_non_trivia(0);
        let second = self.peek_nth_non_trivia(1);
        matches!(
            (first, second),
            (Some(SyntaxKind::WORD), Some(SyntaxKind::SECTION_KW))
        )
    }

    /// Is the current position at a paragraph header (WORD followed by DOT, but not WORD SECTION)?
    fn at_paragraph_header(&self) -> bool {
        let first = self.peek_nth_non_trivia(0);
        let second = self.peek_nth_non_trivia(1);
        if first == Some(SyntaxKind::WORD) && second == Some(SyntaxKind::DOT) {
            // Make sure it's not a section header
            let third = self.peek_nth_non_trivia(2);
            // If the word is followed by DOT and the next thing is a statement start or another
            // paragraph name, it's a paragraph header
            third != Some(SyntaxKind::SECTION_KW)
        } else {
            false
        }
    }

    // ──────────────────────────────────────────────
    // Top-level
    // ──────────────────────────────────────────────

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
        if self.at(SyntaxKind::ENVIRONMENT_KW) {
            self.parse_environment_division();
        }
        if self.at(SyntaxKind::DATA_KW) {
            self.parse_data_division();
        }
        if self.at(SyntaxKind::PROCEDURE_KW) {
            self.parse_procedure_division();
        }
        self.builder.finish_node();
    }

    // ──────────────────────────────────────────────
    // IDENTIFICATION DIVISION
    // ──────────────────────────────────────────────

    fn parse_identification_division(&mut self) {
        self.builder
            .start_node(SyntaxKind::IDENTIFICATION_DIVISION.into());
        self.expect(SyntaxKind::IDENTIFICATION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DIVISION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        self.bump_trivia();
        if self.at(SyntaxKind::PROGRAM_ID_KW) {
            self.parse_program_id_clause();
        }
        // Skip any remaining content until next division
        self.skip_to_division_or_end();
        self.builder.finish_node();
    }

    fn parse_program_id_clause(&mut self) {
        self.builder
            .start_node(SyntaxKind::PROGRAM_ID_CLAUSE.into());
        self.expect(SyntaxKind::PROGRAM_ID_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        // Program name
        self.bump_trivia();
        if self.pos < self.tokens.len() && !self.at(SyntaxKind::DOT) {
            self.bump_any();
        }
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        self.builder.finish_node();
    }

    /// Skip tokens until we see a division keyword or EOF
    fn skip_to_division_or_end(&mut self) {
        loop {
            self.bump_trivia();
            match self.current() {
                None => break,
                Some(SyntaxKind::ENVIRONMENT_KW)
                | Some(SyntaxKind::DATA_KW)
                | Some(SyntaxKind::PROCEDURE_KW) => break,
                Some(SyntaxKind::IDENTIFICATION_KW) => break,
                _ => {
                    // Consume everything until DOT, then continue checking
                    self.bump_to_dot_inclusive();
                }
            }
        }
    }

    /// Bump tokens until and including the next DOT
    fn bump_to_dot_inclusive(&mut self) {
        loop {
            self.bump_trivia();
            match self.current() {
                None => break,
                Some(SyntaxKind::DOT) => {
                    self.bump_any();
                    break;
                }
                _ => {
                    self.bump_any();
                }
            }
        }
    }

    // ──────────────────────────────────────────────
    // ENVIRONMENT DIVISION
    // ──────────────────────────────────────────────

    fn parse_environment_division(&mut self) {
        self.builder
            .start_node(SyntaxKind::ENVIRONMENT_DIVISION.into());
        self.expect(SyntaxKind::ENVIRONMENT_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DIVISION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        if self.at(SyntaxKind::CONFIGURATION_KW) {
            self.parse_configuration_section();
        }
        self.bump_trivia();
        if self.at(SyntaxKind::INPUT_OUTPUT_KW) {
            self.parse_input_output_section();
        }
        self.builder.finish_node();
    }

    fn parse_configuration_section(&mut self) {
        self.builder
            .start_node(SyntaxKind::CONFIGURATION_SECTION.into());
        self.expect(SyntaxKind::CONFIGURATION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::SECTION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        if self.at(SyntaxKind::SOURCE_COMPUTER_KW) {
            self.parse_source_computer_paragraph();
        }
        self.bump_trivia();
        if self.at(SyntaxKind::OBJECT_COMPUTER_KW) {
            self.parse_object_computer_paragraph();
        }
        self.builder.finish_node();
    }

    fn parse_source_computer_paragraph(&mut self) {
        self.builder
            .start_node(SyntaxKind::SOURCE_COMPUTER_PARAGRAPH.into());
        self.expect(SyntaxKind::SOURCE_COMPUTER_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        // Skip to next DOT (computer name etc.)
        self.bump_to_dot_inclusive();
        self.builder.finish_node();
    }

    fn parse_object_computer_paragraph(&mut self) {
        self.builder
            .start_node(SyntaxKind::OBJECT_COMPUTER_PARAGRAPH.into());
        self.expect(SyntaxKind::OBJECT_COMPUTER_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        // Skip to next DOT
        self.bump_to_dot_inclusive();
        self.builder.finish_node();
    }

    fn parse_input_output_section(&mut self) {
        self.builder
            .start_node(SyntaxKind::INPUT_OUTPUT_SECTION.into());
        self.expect(SyntaxKind::INPUT_OUTPUT_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::SECTION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        if self.at(SyntaxKind::FILE_CONTROL_KW) {
            self.parse_file_control_paragraph();
        }
        self.builder.finish_node();
    }

    fn parse_file_control_paragraph(&mut self) {
        self.builder
            .start_node(SyntaxKind::FILE_CONTROL_PARAGRAPH.into());
        self.expect(SyntaxKind::FILE_CONTROL_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        // Parse SELECT clauses
        self.bump_trivia();
        while self.at(SyntaxKind::SELECT_KW) {
            self.parse_select_clause();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_select_clause(&mut self) {
        self.builder.start_node(SyntaxKind::SELECT_CLAUSE.into());
        self.expect(SyntaxKind::SELECT_KW);
        // Skip to DOT
        self.bump_to_dot_inclusive();
        self.builder.finish_node();
    }

    // ──────────────────────────────────────────────
    // DATA DIVISION
    // ──────────────────────────────────────────────

    fn parse_data_division(&mut self) {
        self.builder.start_node(SyntaxKind::DATA_DIVISION.into());
        self.expect(SyntaxKind::DATA_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DIVISION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        if self.at(SyntaxKind::FILE_KW) {
            self.parse_file_section();
        }
        self.bump_trivia();
        if self.at(SyntaxKind::WORKING_STORAGE_KW) {
            self.parse_working_storage_section();
        }
        self.builder.finish_node();
    }

    fn parse_file_section(&mut self) {
        self.builder.start_node(SyntaxKind::FILE_SECTION.into());
        self.expect(SyntaxKind::FILE_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::SECTION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        while self.at(SyntaxKind::FD_KW) {
            self.parse_fd_entry();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_fd_entry(&mut self) {
        self.builder.start_node(SyntaxKind::FD_ENTRY.into());
        self.expect(SyntaxKind::FD_KW);
        // FD file-name .
        self.bump_to_dot_inclusive();
        // Data description entries
        self.bump_trivia();
        while self.at_level_number() {
            self.parse_data_description_entry();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_working_storage_section(&mut self) {
        self.builder
            .start_node(SyntaxKind::WORKING_STORAGE_SECTION.into());
        self.expect(SyntaxKind::WORKING_STORAGE_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::SECTION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        while self.at_level_number() {
            self.parse_data_description_entry();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_data_description_entry(&mut self) {
        self.builder
            .start_node(SyntaxKind::DATA_DESCRIPTION_ENTRY.into());

        // Level number
        self.bump_any(); // NUMERIC_LITERAL

        // Data name or FILLER
        self.bump_trivia();
        match self.current() {
            Some(SyntaxKind::FILLER_KW) => {
                self.bump_any();
            }
            Some(SyntaxKind::DOT) => {
                // Group item with no name (just level number followed by DOT)
            }
            _ => {
                // Data name (WORD or keyword used as name)
                if self.current().is_some() && self.current() != Some(SyntaxKind::DOT) {
                    // Could be a data name - consume it if it's a word-like token
                    if !self.at(SyntaxKind::PIC_KW)
                        && !self.at(SyntaxKind::PICTURE_KW)
                        && !self.at(SyntaxKind::VALUE_KW)
                        && !self.at(SyntaxKind::REDEFINES_KW)
                        && !self.at(SyntaxKind::COMPUTATIONAL_KW)
                        && !self.at(SyntaxKind::COMP_KW)
                    {
                        self.bump_any();
                    }
                }
            }
        }

        // Parse clauses until DOT
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) {
            match self.current() {
                Some(SyntaxKind::PIC_KW) | Some(SyntaxKind::PICTURE_KW) => {
                    self.parse_picture_clause();
                }
                Some(SyntaxKind::VALUE_KW) => {
                    self.parse_value_clause();
                }
                Some(SyntaxKind::REDEFINES_KW) => {
                    self.parse_redefines_clause();
                }
                Some(SyntaxKind::COMPUTATIONAL_KW) | Some(SyntaxKind::COMP_KW) => {
                    self.parse_usage_clause();
                }
                _ => {
                    // Skip unknown tokens
                    self.bump_any();
                }
            }
            self.bump_trivia();
        }

        // Trailing DOT
        self.eat(SyntaxKind::DOT);

        self.builder.finish_node();
    }

    fn parse_picture_clause(&mut self) {
        self.builder.start_node(SyntaxKind::PICTURE_CLAUSE.into());
        // PIC or PICTURE
        self.bump_any();
        // Optional IS
        self.bump_trivia();
        if self.at(SyntaxKind::IS_KW) {
            self.bump_any();
        }
        // Picture string: consume tokens until we hit a clause boundary
        // Picture strings can contain: X, 9, A, V, S, P, Z, *, +, -, $, ., ,, /, B, 0, CR, DB
        // and parenthesized repeat counts like (18)
        self.bump_trivia();
        self.consume_picture_string();
        self.builder.finish_node();
    }

    fn consume_picture_string(&mut self) {
        // Consume tokens that form the picture string
        // Stop at: VALUE, REDEFINES, COMPUTATIONAL, COMP, DOT, level number, or other clause keywords
        while self.current().is_some() && !self.at(SyntaxKind::DOT) {
            match self.current() {
                Some(SyntaxKind::VALUE_KW)
                | Some(SyntaxKind::REDEFINES_KW)
                | Some(SyntaxKind::COMPUTATIONAL_KW)
                | Some(SyntaxKind::COMP_KW) => break,
                Some(SyntaxKind::LPAREN) => {
                    // Consume (n) repeat
                    self.bump_any(); // (
                    self.bump_trivia();
                    if self.at(SyntaxKind::NUMERIC_LITERAL) {
                        self.bump_any();
                    }
                    self.bump_trivia();
                    if self.at(SyntaxKind::RPAREN) {
                        self.bump_any();
                    }
                    self.bump_trivia();
                }
                _ => {
                    self.bump_any();
                    self.bump_trivia();
                }
            }
        }
    }

    fn parse_value_clause(&mut self) {
        self.builder.start_node(SyntaxKind::VALUE_CLAUSE.into());
        self.expect(SyntaxKind::VALUE_KW);
        // Optional IS
        self.bump_trivia();
        if self.at(SyntaxKind::IS_KW) {
            self.bump_any();
        }
        // Value: literal, figurative constant, etc.
        self.bump_trivia();
        // Consume value tokens until clause boundary
        while self.current().is_some() && !self.at(SyntaxKind::DOT) {
            match self.current() {
                Some(SyntaxKind::PIC_KW)
                | Some(SyntaxKind::PICTURE_KW)
                | Some(SyntaxKind::REDEFINES_KW)
                | Some(SyntaxKind::COMPUTATIONAL_KW)
                | Some(SyntaxKind::COMP_KW) => break,
                _ => {
                    self.bump_any();
                    self.bump_trivia();
                }
            }
        }
        self.builder.finish_node();
    }

    fn parse_redefines_clause(&mut self) {
        self.builder.start_node(SyntaxKind::REDEFINES_CLAUSE.into());
        self.expect(SyntaxKind::REDEFINES_KW);
        // Data name
        self.bump_trivia();
        if self.current().is_some() && !self.at(SyntaxKind::DOT) {
            self.bump_any();
        }
        self.builder.finish_node();
    }

    fn parse_usage_clause(&mut self) {
        self.builder.start_node(SyntaxKind::USAGE_CLAUSE.into());
        // COMPUTATIONAL or COMP
        self.bump_any();
        self.builder.finish_node();
    }

    // ──────────────────────────────────────────────
    // PROCEDURE DIVISION
    // ──────────────────────────────────────────────

    fn parse_procedure_division(&mut self) {
        self.builder
            .start_node(SyntaxKind::PROCEDURE_DIVISION.into());
        self.expect(SyntaxKind::PROCEDURE_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DIVISION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        self.bump_trivia();
        self.parse_procedure_body();

        self.builder.finish_node();
    }

    fn parse_procedure_body(&mut self) {
        while self.current().is_some() {
            if self.at_section_header() {
                self.parse_section();
            } else if self.at_paragraph_header() {
                self.parse_paragraph();
            } else if self.at_statement_start() {
                self.parse_sentence();
            } else {
                break;
            }
            self.bump_trivia();
        }
    }

    fn parse_section(&mut self) {
        self.builder.start_node(SyntaxKind::SECTION_HEADER.into());
        // Section name (WORD)
        self.bump_any();
        // SECTION
        self.bump_trivia();
        self.expect(SyntaxKind::SECTION_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);
        self.builder.finish_node();

        // Parse paragraphs and sentences within the section
        self.bump_trivia();
        while self.current().is_some() {
            if self.at_section_header() {
                break; // Next section
            } else if self.at_paragraph_header() {
                self.parse_paragraph();
            } else if self.at_statement_start() {
                self.parse_sentence();
            } else {
                break;
            }
            self.bump_trivia();
        }
    }

    fn parse_paragraph(&mut self) {
        self.builder.start_node(SyntaxKind::PARAGRAPH.into());
        // Paragraph name
        self.builder.start_node(SyntaxKind::PARAGRAPH_NAME.into());
        self.bump_any(); // WORD
        self.builder.finish_node();
        self.bump_trivia();
        self.expect(SyntaxKind::DOT);

        // Sentences
        self.bump_trivia();
        while self.current().is_some() && !self.at_section_header() && !self.at_paragraph_header() {
            if self.at_statement_start() {
                self.parse_sentence();
                self.bump_trivia();
            } else {
                break;
            }
        }

        self.builder.finish_node();
    }

    fn parse_sentence(&mut self) {
        self.builder.start_node(SyntaxKind::SENTENCE.into());

        // Parse statements until DOT
        while self.current().is_some() && !self.at(SyntaxKind::DOT) {
            if self.at_statement_start() {
                self.parse_statement();
                self.bump_trivia();
            } else {
                break;
            }
        }

        // Trailing DOT
        self.bump_trivia();
        self.eat(SyntaxKind::DOT);

        self.builder.finish_node();
    }

    fn parse_statement(&mut self) {
        match self.current() {
            Some(SyntaxKind::DISPLAY_KW) => self.parse_display_statement(),
            Some(SyntaxKind::STOP_KW) => self.parse_stop_statement(),
            Some(SyntaxKind::MOVE_KW) => self.parse_move_statement(),
            Some(SyntaxKind::PERFORM_KW) => self.parse_perform_statement(),
            Some(SyntaxKind::GO_KW) => self.parse_go_to_statement(),
            Some(SyntaxKind::IF_KW) => self.parse_if_statement(),
            Some(SyntaxKind::ADD_KW) => self.parse_add_statement(),
            Some(SyntaxKind::MULTIPLY_KW) => self.parse_multiply_statement(),
            Some(SyntaxKind::WRITE_KW) => self.parse_write_statement(),
            Some(SyntaxKind::OPEN_KW) => self.parse_open_statement(),
            Some(SyntaxKind::CLOSE_KW) => self.parse_close_statement(),
            Some(SyntaxKind::EXIT_KW) => self.parse_exit_statement(),
            _ => {}
        }
    }

    fn parse_display_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::DISPLAY_STATEMENT.into());
        self.expect(SyntaxKind::DISPLAY_KW);
        // Operands until DOT or statement boundary
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_stop_statement(&mut self) {
        self.builder.start_node(SyntaxKind::STOP_STATEMENT.into());
        self.expect(SyntaxKind::STOP_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::RUN_KW);
        self.builder.finish_node();
    }

    fn parse_move_statement(&mut self) {
        self.builder.start_node(SyntaxKind::MOVE_STATEMENT.into());
        self.expect(SyntaxKind::MOVE_KW);
        // Source operand(s) - everything until TO
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::TO_KW) && !self.at(SyntaxKind::DOT) {
            self.bump_any();
            self.bump_trivia();
        }
        // TO
        self.eat(SyntaxKind::TO_KW);
        // Target operand(s)
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_perform_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::PERFORM_STATEMENT.into());
        self.expect(SyntaxKind::PERFORM_KW);
        // Paragraph/section name
        self.bump_trivia();
        if self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
        }
        // Optional THRU/THROUGH
        self.bump_trivia();
        if self.at(SyntaxKind::THRU_KW) || self.at(SyntaxKind::THROUGH_KW) {
            self.bump_any();
            // End paragraph name
            self.bump_trivia();
            if self.current().is_some()
                && !self.at(SyntaxKind::DOT)
                && !self.at_statement_start()
                && !self.at(SyntaxKind::TIMES_KW)
            {
                self.bump_any();
            }
        }
        // Optional n TIMES
        self.bump_trivia();
        if self.at(SyntaxKind::NUMERIC_LITERAL) {
            // Check if followed by TIMES
            if self.peek_nth_non_trivia(1) == Some(SyntaxKind::TIMES_KW) {
                self.bump_any(); // number
                self.bump_trivia();
                self.bump_any(); // TIMES
            }
        }
        self.builder.finish_node();
    }

    fn parse_go_to_statement(&mut self) {
        self.builder.start_node(SyntaxKind::GO_TO_STATEMENT.into());
        self.expect(SyntaxKind::GO_KW);
        self.bump_trivia();
        self.eat(SyntaxKind::TO_KW);
        // Target paragraph name
        self.bump_trivia();
        if self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
        }
        self.builder.finish_node();
    }

    fn parse_if_statement(&mut self) {
        self.builder.start_node(SyntaxKind::IF_STATEMENT.into());
        self.expect(SyntaxKind::IF_KW);

        // Parse condition
        self.bump_trivia();
        self.parse_condition();

        // Parse then-branch statements
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at(SyntaxKind::ELSE_KW)
        {
            if self.at_statement_start() {
                self.parse_statement();
                self.bump_trivia();
            } else {
                break;
            }
        }

        // Optional ELSE
        if self.at(SyntaxKind::ELSE_KW) {
            self.parse_else_clause();
        }

        self.builder.finish_node();
    }

    fn parse_condition(&mut self) {
        self.builder.start_node(SyntaxKind::CONDITION.into());
        // Consume tokens until we hit a statement-starting keyword or DOT or ELSE
        // A condition looks like: operand [NOT] EQUAL [TO] operand
        //                     or: operand [IS] [NOT] EQUAL [TO] operand
        //                     or: operand = operand
        //                     or: operand GREATER [THAN] operand
        while self.current().is_some()
            && !self.at(SyntaxKind::DOT)
            && !self.at_statement_start()
            && !self.at(SyntaxKind::ELSE_KW)
        {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_else_clause(&mut self) {
        self.builder.start_node(SyntaxKind::ELSE_CLAUSE.into());
        self.expect(SyntaxKind::ELSE_KW);
        // Parse else-branch statements until DOT
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) {
            if self.at_statement_start() {
                self.parse_statement();
                self.bump_trivia();
            } else {
                break;
            }
        }
        self.builder.finish_node();
    }

    fn parse_add_statement(&mut self) {
        self.builder.start_node(SyntaxKind::ADD_STATEMENT.into());
        self.expect(SyntaxKind::ADD_KW);
        // Source operand(s) until TO
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::TO_KW) && !self.at(SyntaxKind::DOT) {
            self.bump_any();
            self.bump_trivia();
        }
        // TO
        self.eat(SyntaxKind::TO_KW);
        // Target operand(s)
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_multiply_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::MULTIPLY_STATEMENT.into());
        self.expect(SyntaxKind::MULTIPLY_KW);
        // Source operand until BY
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::BY_KW) && !self.at(SyntaxKind::DOT) {
            self.bump_any();
            self.bump_trivia();
        }
        // BY
        self.eat(SyntaxKind::BY_KW);
        // Target operand(s)
        self.bump_trivia();
        while self.current().is_some()
            && !self.at(SyntaxKind::DOT)
            && !self.at(SyntaxKind::ROUNDED_KW)
            && !self.at(SyntaxKind::ON_KW)
            && !self.at(SyntaxKind::NOT_KW)
            && !self.at(SyntaxKind::END_MULTIPLY_KW)
            && !self.at_statement_start()
        {
            self.bump_any();
            self.bump_trivia();
        }
        // Optional ROUNDED
        if self.at(SyntaxKind::ROUNDED_KW) {
            self.bump_any();
            self.bump_trivia();
        }
        // Optional ON SIZE ERROR
        if self.at(SyntaxKind::ON_KW) && self.peek_nth_non_trivia(1) == Some(SyntaxKind::SIZE_KW) {
            self.parse_on_size_error_clause();
            self.bump_trivia();
        }
        // Optional NOT ON SIZE ERROR
        if self.at(SyntaxKind::NOT_KW)
            && (self.peek_nth_non_trivia(1) == Some(SyntaxKind::ON_KW)
                || self.peek_nth_non_trivia(1) == Some(SyntaxKind::SIZE_KW))
        {
            self.parse_not_on_size_error_clause();
            self.bump_trivia();
        }
        // Optional END-MULTIPLY
        if self.at(SyntaxKind::END_MULTIPLY_KW) {
            self.bump_any();
        }
        self.builder.finish_node();
    }

    fn parse_on_size_error_clause(&mut self) {
        self.builder
            .start_node(SyntaxKind::ON_SIZE_ERROR_CLAUSE.into());
        self.expect(SyntaxKind::ON_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::SIZE_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::ERROR_KW);
        // Statements
        self.bump_trivia();
        while self.current().is_some()
            && !self.at(SyntaxKind::DOT)
            && !self.at(SyntaxKind::NOT_KW)
            && !self.at(SyntaxKind::END_MULTIPLY_KW)
        {
            if self.at_statement_start() {
                self.parse_statement();
                self.bump_trivia();
            } else {
                break;
            }
        }
        self.builder.finish_node();
    }

    fn parse_not_on_size_error_clause(&mut self) {
        self.builder
            .start_node(SyntaxKind::NOT_ON_SIZE_ERROR_CLAUSE.into());
        self.expect(SyntaxKind::NOT_KW);
        self.bump_trivia();
        // ON is optional in NOT ON SIZE ERROR
        if self.at(SyntaxKind::ON_KW) {
            self.bump_any();
            self.bump_trivia();
        }
        self.expect(SyntaxKind::SIZE_KW);
        self.bump_trivia();
        self.expect(SyntaxKind::ERROR_KW);
        // Statements
        self.bump_trivia();
        while self.current().is_some()
            && !self.at(SyntaxKind::DOT)
            && !self.at(SyntaxKind::END_MULTIPLY_KW)
        {
            if self.at_statement_start() {
                self.parse_statement();
                self.bump_trivia();
            } else {
                break;
            }
        }
        self.builder.finish_node();
    }

    fn parse_write_statement(&mut self) {
        self.builder.start_node(SyntaxKind::WRITE_STATEMENT.into());
        self.expect(SyntaxKind::WRITE_KW);
        // Record name
        self.bump_trivia();
        if self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at(SyntaxKind::AFTER_KW) {
            self.bump_any();
        }
        // Optional AFTER ADVANCING
        self.bump_trivia();
        if self.at(SyntaxKind::AFTER_KW) {
            self.parse_advancing_clause();
        }
        self.builder.finish_node();
    }

    fn parse_advancing_clause(&mut self) {
        self.builder.start_node(SyntaxKind::ADVANCING_CLAUSE.into());
        self.expect(SyntaxKind::AFTER_KW);
        self.bump_trivia();
        if self.at(SyntaxKind::ADVANCING_KW) {
            self.bump_any();
        }
        // PAGE, n LINES, n LINE, identifier
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_open_statement(&mut self) {
        self.builder.start_node(SyntaxKind::OPEN_STATEMENT.into());
        self.expect(SyntaxKind::OPEN_KW);
        // OUTPUT/INPUT + file name(s)
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_close_statement(&mut self) {
        self.builder.start_node(SyntaxKind::CLOSE_STATEMENT.into());
        self.expect(SyntaxKind::CLOSE_KW);
        // File name(s)
        self.bump_trivia();
        while self.current().is_some() && !self.at(SyntaxKind::DOT) && !self.at_statement_start() {
            self.bump_any();
            self.bump_trivia();
        }
        self.builder.finish_node();
    }

    fn parse_exit_statement(&mut self) {
        self.builder.start_node(SyntaxKind::EXIT_STATEMENT.into());
        self.expect(SyntaxKind::EXIT_KW);
        self.builder.finish_node();
    }
}
