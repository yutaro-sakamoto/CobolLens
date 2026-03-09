pub mod ast;
pub mod language;
pub mod lexer;
pub mod parser;
pub mod syntax_kind;

#[cfg(test)]
mod tests {
    use crate::ast::SourceFile;
    use crate::lexer::lex;
    use crate::parser::parse;
    use crate::syntax_kind::SyntaxKind;

    const HELLO_WORLD: &str = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       PROCEDURE DIVISION.
       DISPLAY \"HELLO WORLD\".
       STOP RUN.
";

    #[test]
    fn lex_keywords() {
        let tokens = lex("IDENTIFICATION DIVISION");
        assert_eq!(tokens[0].0, SyntaxKind::IDENTIFICATION_KW);
        assert_eq!(tokens[2].0, SyntaxKind::DIVISION_KW);
    }

    #[test]
    fn lex_case_insensitive() {
        let tokens = lex("identification division");
        assert_eq!(tokens[0].0, SyntaxKind::IDENTIFICATION_KW);
        assert_eq!(tokens[2].0, SyntaxKind::DIVISION_KW);
    }

    #[test]
    fn lex_program_id() {
        let tokens = lex("PROGRAM-ID");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, SyntaxKind::PROGRAM_ID_KW);
        assert_eq!(tokens[0].1, "PROGRAM-ID");
    }

    #[test]
    fn lex_string_literal() {
        let tokens = lex("\"HELLO WORLD\"");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, SyntaxKind::STRING_LITERAL);
        assert_eq!(tokens[0].1, "\"HELLO WORLD\"");
    }

    #[test]
    fn lex_single_quote_string() {
        let tokens = lex("'HELLO'");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].0, SyntaxKind::STRING_LITERAL);
    }

    #[test]
    fn parse_hello_world_no_errors() {
        let tree = parse(HELLO_WORLD);
        assert_eq!(tree.kind(), SyntaxKind::SOURCE_FILE);
        // No ERROR tokens in the tree
        let errors: Vec<_> = tree
            .descendants_with_tokens()
            .filter(|el| el.kind() == SyntaxKind::ERROR)
            .collect();
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn parse_tree_structure() {
        let tree = parse(HELLO_WORLD);
        let source = SourceFile::cast(tree).unwrap();
        let program = source.program().expect("PROGRAM_DEFINITION");
        let id_div = program
            .identification_division()
            .expect("IDENTIFICATION_DIVISION");
        let pid = id_div.program_id_clause().expect("PROGRAM_ID_CLAUSE");
        assert_eq!(pid.name().unwrap().text(), "HELLO");

        let proc_div = program.procedure_division().expect("PROCEDURE_DIVISION");
        let sentences: Vec<_> = proc_div.sentences().collect();
        assert_eq!(sentences.len(), 2);
    }

    #[test]
    fn parse_lossless() {
        let tree = parse(HELLO_WORLD);
        assert_eq!(tree.text().to_string(), HELLO_WORLD);
    }
}
