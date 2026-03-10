use crate::syntax_kind::SyntaxKind;

pub fn lex(input: &str) -> Vec<(SyntaxKind, String)> {
    let mut tokens = Vec::new();
    let mut rest = input;

    while !rest.is_empty() {
        let (line, remainder) = match rest.find('\n') {
            Some(pos) => (&rest[..pos], &rest[pos + 1..]),
            None => (rest, ""),
        };
        let has_newline = rest.len() != line.len();

        lex_line(line, &mut tokens);

        if has_newline {
            tokens.push((SyntaxKind::NEWLINE, "\n".to_string()));
        }

        rest = remainder;
    }

    tokens
}

fn lex_line(line: &str, tokens: &mut Vec<(SyntaxKind, String)>) {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if len == 0 {
        return;
    }

    // Fixed format: columns 1-6 = sequence number, 7 = indicator, 8-72 = code, 73+ = identification area
    if len >= 7 {
        // Columns 1-6: sequence number
        let seq: String = chars[..6].iter().collect();
        tokens.push((SyntaxKind::SEQUENCE_NUMBER, seq));

        // Column 7: indicator
        let indicator = chars[6];
        tokens.push((SyntaxKind::INDICATOR, indicator.to_string()));

        // Determine code area end (column 72 = index 71)
        let code_end = if len > 72 { 72 } else { len };
        let code: String = chars[7..code_end].iter().collect();

        if indicator == '*' {
            // Comment line - rest is comment text
            tokens.push((SyntaxKind::COMMENT, code));
        } else {
            // Normal or continuation line - lex the code area
            lex_code(&code, tokens);
        }

        // Columns 73+: identification area
        if len > 72 {
            let id_area: String = chars[72..].iter().collect();
            tokens.push((SyntaxKind::IDENTIFICATION_AREA, id_area));
        }
    } else {
        // Short line - emit as whitespace
        let s: String = chars.iter().collect();
        if !s.is_empty() {
            tokens.push((SyntaxKind::WHITESPACE, s));
        }
    }
}

fn lex_code(input: &str, tokens: &mut Vec<(SyntaxKind, String)>) {
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ' ' || c == '\t' || c == '\r' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push((SyntaxKind::WHITESPACE, s));
            }
            '.' => {
                // Check if this is a decimal point starting a numeric literal (e.g., .11111)
                let mut lookahead = chars.clone();
                lookahead.next(); // skip the '.'
                if let Some(&next) = lookahead.peek()
                    && next.is_ascii_digit()
                {
                    // It's a numeric literal starting with '.'
                    let mut s = String::new();
                    s.push('.');
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            s.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push((SyntaxKind::NUMERIC_LITERAL, s));
                    continue;
                }
                chars.next();
                tokens.push((SyntaxKind::DOT, ".".to_string()));
            }
            '"' | '\'' => {
                let quote = c;
                let mut s = String::new();
                s.push(quote);
                chars.next();
                while let Some(&c) = chars.peek() {
                    s.push(c);
                    chars.next();
                    if c == quote {
                        break;
                    }
                }
                tokens.push((SyntaxKind::STRING_LITERAL, s));
            }
            '(' => {
                chars.next();
                tokens.push((SyntaxKind::LPAREN, "(".to_string()));
            }
            ')' => {
                chars.next();
                tokens.push((SyntaxKind::RPAREN, ")".to_string()));
            }
            '=' => {
                chars.next();
                tokens.push((SyntaxKind::EQUALS_SIGN, "=".to_string()));
            }
            ',' => {
                chars.next();
                tokens.push((SyntaxKind::COMMA, ",".to_string()));
            }
            ';' => {
                chars.next();
                tokens.push((SyntaxKind::SEMICOLON, ";".to_string()));
            }
            '+' | '-' => {
                // Check if this is a signed numeric literal
                let sign = c;
                let mut lookahead = chars.clone();
                lookahead.next(); // skip sign
                if let Some(&next) = lookahead.peek()
                    && (next.is_ascii_digit() || next == '.')
                {
                    // Signed numeric literal
                    let mut s = String::new();
                    s.push(sign);
                    chars.next();
                    let mut has_dot = false;
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            s.push(c);
                            chars.next();
                        } else if c == '.' && !has_dot {
                            // Check if next char is digit (decimal point)
                            let mut la = chars.clone();
                            la.next();
                            if let Some(&nc) = la.peek() {
                                if nc.is_ascii_digit() {
                                    has_dot = true;
                                    s.push(c);
                                    chars.next();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    tokens.push((SyntaxKind::NUMERIC_LITERAL, s));
                    continue;
                }
                chars.next();
                let kind = if sign == '+' {
                    SyntaxKind::PLUS
                } else {
                    SyntaxKind::MINUS
                };
                tokens.push((kind, sign.to_string()));
            }
            _ if c.is_ascii_digit() => {
                let mut s = String::new();
                let mut has_dot = false;
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        s.push(c);
                        chars.next();
                    } else if c == '.' && !has_dot {
                        // Peek ahead to see if it's a decimal point
                        let mut la = chars.clone();
                        la.next();
                        if let Some(&nc) = la.peek() {
                            if nc.is_ascii_digit() {
                                has_dot = true;
                                s.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                tokens.push((SyntaxKind::NUMERIC_LITERAL, s));
            }
            _ if c.is_ascii_alphabetic() || c == '-' || c == '_' => {
                // Special case: standalone '-' not followed by alphanumeric
                if c == '-' {
                    let mut la = chars.clone();
                    la.next();
                    match la.peek() {
                        Some(&nc) if nc.is_ascii_alphanumeric() || nc == '-' || nc == '_' => {}
                        _ => {
                            chars.next();
                            tokens.push((SyntaxKind::MINUS, "-".to_string()));
                            continue;
                        }
                    }
                }
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = keyword_kind(&s);
                tokens.push((kind, s));
            }
            '$' | '*' | '/' | '#' | '&' | '@' | '!' | '%' | '^' | '~' | '\\' | '`' | '{' | '}'
            | '[' | ']' | '|' | ':' | '<' | '>' | '?' => {
                chars.next();
                let kind = match c {
                    '*' => SyntaxKind::STAR,
                    '/' => SyntaxKind::SLASH,
                    _ => SyntaxKind::WORD,
                };
                tokens.push((kind, c.to_string()));
            }
            _ => {
                chars.next();
                tokens.push((SyntaxKind::WORD, c.to_string()));
            }
        }
    }
}

fn keyword_kind(word: &str) -> SyntaxKind {
    match word.to_ascii_uppercase().as_str() {
        "IDENTIFICATION" => SyntaxKind::IDENTIFICATION_KW,
        "DIVISION" => SyntaxKind::DIVISION_KW,
        "PROGRAM-ID" => SyntaxKind::PROGRAM_ID_KW,
        "PROCEDURE" => SyntaxKind::PROCEDURE_KW,
        "DISPLAY" => SyntaxKind::DISPLAY_KW,
        "STOP" => SyntaxKind::STOP_KW,
        "RUN" => SyntaxKind::RUN_KW,
        "ENVIRONMENT" => SyntaxKind::ENVIRONMENT_KW,
        "CONFIGURATION" => SyntaxKind::CONFIGURATION_KW,
        "SOURCE-COMPUTER" => SyntaxKind::SOURCE_COMPUTER_KW,
        "OBJECT-COMPUTER" => SyntaxKind::OBJECT_COMPUTER_KW,
        "INPUT-OUTPUT" => SyntaxKind::INPUT_OUTPUT_KW,
        "FILE-CONTROL" => SyntaxKind::FILE_CONTROL_KW,
        "DATA" => SyntaxKind::DATA_KW,
        "FILE" => SyntaxKind::FILE_KW,
        "FD" => SyntaxKind::FD_KW,
        "WORKING-STORAGE" => SyntaxKind::WORKING_STORAGE_KW,
        "SECTION" => SyntaxKind::SECTION_KW,
        "SELECT" => SyntaxKind::SELECT_KW,
        "ASSIGN" => SyntaxKind::ASSIGN_KW,
        "TO" => SyntaxKind::TO_KW,
        "PICTURE" => SyntaxKind::PICTURE_KW,
        "PIC" => SyntaxKind::PIC_KW,
        "VALUE" => SyntaxKind::VALUE_KW,
        "REDEFINES" => SyntaxKind::REDEFINES_KW,
        "COMPUTATIONAL" => SyntaxKind::COMPUTATIONAL_KW,
        "COMP" => SyntaxKind::COMP_KW,
        "FILLER" => SyntaxKind::FILLER_KW,
        "IS" => SyntaxKind::IS_KW,
        "ZERO" => SyntaxKind::ZERO_KW,
        "ZEROS" | "ZEROES" => SyntaxKind::ZEROS_KW,
        "SPACE" => SyntaxKind::SPACE_KW,
        "SPACES" => SyntaxKind::SPACES_KW,
        "OPEN" => SyntaxKind::OPEN_KW,
        "OUTPUT" => SyntaxKind::OUTPUT_KW,
        "INPUT" => SyntaxKind::INPUT_KW,
        "CLOSE" => SyntaxKind::CLOSE_KW,
        "MOVE" => SyntaxKind::MOVE_KW,
        "PERFORM" => SyntaxKind::PERFORM_KW,
        "THRU" => SyntaxKind::THRU_KW,
        "THROUGH" => SyntaxKind::THROUGH_KW,
        "TIMES" => SyntaxKind::TIMES_KW,
        "GO" => SyntaxKind::GO_KW,
        "IF" => SyntaxKind::IF_KW,
        "ELSE" => SyntaxKind::ELSE_KW,
        "NOT" => SyntaxKind::NOT_KW,
        "EQUAL" => SyntaxKind::EQUAL_KW,
        "GREATER" => SyntaxKind::GREATER_KW,
        "LESS" => SyntaxKind::LESS_KW,
        "THAN" => SyntaxKind::THAN_KW,
        "ADD" => SyntaxKind::ADD_KW,
        "MULTIPLY" => SyntaxKind::MULTIPLY_KW,
        "BY" => SyntaxKind::BY_KW,
        "ROUNDED" => SyntaxKind::ROUNDED_KW,
        "ON" => SyntaxKind::ON_KW,
        "SIZE" => SyntaxKind::SIZE_KW,
        "ERROR" => SyntaxKind::ERROR_KW,
        "END-MULTIPLY" => SyntaxKind::END_MULTIPLY_KW,
        "WRITE" => SyntaxKind::WRITE_KW,
        "AFTER" => SyntaxKind::AFTER_KW,
        "ADVANCING" => SyntaxKind::ADVANCING_KW,
        "PAGE" => SyntaxKind::PAGE_KW,
        "LINES" => SyntaxKind::LINES_KW,
        "LINE" => SyntaxKind::LINE_KW,
        "EXIT" => SyntaxKind::EXIT_KW,
        "RECORD" => SyntaxKind::RECORD_KW,
        "OPTIONAL" => SyntaxKind::OPTIONAL_KW,
        "DELETE" => SyntaxKind::DELETE_KW,
        _ => SyntaxKind::WORD,
    }
}
