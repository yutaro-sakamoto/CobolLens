use crate::syntax_kind::SyntaxKind;

pub fn lex(input: &str) -> Vec<(SyntaxKind, String)> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '\n' => {
                chars.next();
                tokens.push((SyntaxKind::NEWLINE, "\n".to_string()));
            }
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
            _ if c.is_ascii_alphabetic() || c == '-' => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '-' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = keyword_kind(&s);
                tokens.push((kind, s));
            }
            _ if c.is_ascii_digit() => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '-' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = keyword_kind(&s);
                tokens.push((kind, s));
            }
            _ => {
                chars.next();
                tokens.push((SyntaxKind::ERROR, c.to_string()));
            }
        }
    }

    tokens
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
        _ => SyntaxKind::WORD,
    }
}
