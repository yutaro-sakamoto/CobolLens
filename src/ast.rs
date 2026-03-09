use crate::language::{SyntaxNode, SyntaxToken};
use crate::syntax_kind::SyntaxKind;

macro_rules! ast_node {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            syntax: SyntaxNode,
        }

        impl $name {
            pub fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self { syntax: node })
                } else {
                    None
                }
            }

            #[allow(dead_code)]
            pub fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }
    };
}

ast_node!(SourceFile, SyntaxKind::SOURCE_FILE);
ast_node!(ProgramDefinition, SyntaxKind::PROGRAM_DEFINITION);
ast_node!(IdentificationDivision, SyntaxKind::IDENTIFICATION_DIVISION);
ast_node!(ProgramIdClause, SyntaxKind::PROGRAM_ID_CLAUSE);
ast_node!(ProcedureDivision, SyntaxKind::PROCEDURE_DIVISION);
ast_node!(Sentence, SyntaxKind::SENTENCE);
ast_node!(DisplayStatement, SyntaxKind::DISPLAY_STATEMENT);
ast_node!(StopStatement, SyntaxKind::STOP_STATEMENT);

impl SourceFile {
    pub fn program(&self) -> Option<ProgramDefinition> {
        self.syntax.children().find_map(ProgramDefinition::cast)
    }
}

impl ProgramDefinition {
    pub fn identification_division(&self) -> Option<IdentificationDivision> {
        self.syntax.children().find_map(IdentificationDivision::cast)
    }

    pub fn procedure_division(&self) -> Option<ProcedureDivision> {
        self.syntax.children().find_map(ProcedureDivision::cast)
    }
}

impl IdentificationDivision {
    pub fn program_id_clause(&self) -> Option<ProgramIdClause> {
        self.syntax.children().find_map(ProgramIdClause::cast)
    }
}

impl ProgramIdClause {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.syntax
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find(|tok| tok.kind() == SyntaxKind::WORD)
    }
}

impl ProcedureDivision {
    pub fn sentences(&self) -> impl Iterator<Item = Sentence> {
        self.syntax.children().filter_map(Sentence::cast)
    }
}
