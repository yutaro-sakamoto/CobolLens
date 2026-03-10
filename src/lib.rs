pub mod ast;
pub mod language;
pub mod lexer;
pub mod parser;
pub mod syntax_kind;

#[cfg(test)]
mod tests {
    use crate::ast::SourceFile;
    use crate::parser::parse;
    use crate::syntax_kind::SyntaxKind;

    // Fixed-format Hello World (columns: 1-6 seq, 7 indicator, 8-72 code)
    const HELLO_WORLD: &str = "\
000100 IDENTIFICATION DIVISION.
000200 PROGRAM-ID. HELLO.
000300 PROCEDURE DIVISION.
000400 DISPLAY \"HELLO WORLD\".
000500 STOP RUN.
";

    #[test]
    fn parse_hello_world_no_errors() {
        let tree = parse(HELLO_WORLD);
        assert_eq!(tree.kind(), SyntaxKind::SOURCE_FILE);
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

    #[test]
    fn parse_nc101a() {
        let input =
            std::fs::read_to_string("opensourcecobol4j/tests/cobol85/NC/NC101A.CBL").unwrap();
        let tree = parse(&input);

        // Lossless
        assert_eq!(tree.text().to_string(), input, "parse is not lossless");

        // No ERROR tokens
        let errors: Vec<_> = tree
            .descendants_with_tokens()
            .filter(|el| el.kind() == SyntaxKind::ERROR)
            .collect();
        assert!(
            errors.is_empty(),
            "unexpected ERROR tokens: {} found\nfirst few: {:?}",
            errors.len(),
            &errors[..errors.len().min(5)]
        );

        // Root structure
        assert_eq!(tree.kind(), SyntaxKind::SOURCE_FILE);
        let source = SourceFile::cast(tree).unwrap();
        let program = source.program().expect("PROGRAM_DEFINITION");
        program
            .identification_division()
            .expect("IDENTIFICATION_DIVISION");
        program.procedure_division().expect("PROCEDURE_DIVISION");
    }
}
