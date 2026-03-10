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

// New nodes
ast_node!(EnvironmentDivision, SyntaxKind::ENVIRONMENT_DIVISION);
ast_node!(ConfigurationSection, SyntaxKind::CONFIGURATION_SECTION);
ast_node!(
    SourceComputerParagraph,
    SyntaxKind::SOURCE_COMPUTER_PARAGRAPH
);
ast_node!(
    ObjectComputerParagraph,
    SyntaxKind::OBJECT_COMPUTER_PARAGRAPH
);
ast_node!(InputOutputSection, SyntaxKind::INPUT_OUTPUT_SECTION);
ast_node!(FileControlParagraph, SyntaxKind::FILE_CONTROL_PARAGRAPH);
ast_node!(SelectClause, SyntaxKind::SELECT_CLAUSE);
ast_node!(DataDivision, SyntaxKind::DATA_DIVISION);
ast_node!(FileSection, SyntaxKind::FILE_SECTION);
ast_node!(FdEntry, SyntaxKind::FD_ENTRY);
ast_node!(WorkingStorageSection, SyntaxKind::WORKING_STORAGE_SECTION);
ast_node!(DataDescriptionEntry, SyntaxKind::DATA_DESCRIPTION_ENTRY);
ast_node!(PictureClause, SyntaxKind::PICTURE_CLAUSE);
ast_node!(ValueClause, SyntaxKind::VALUE_CLAUSE);
ast_node!(RedefinesClause, SyntaxKind::REDEFINES_CLAUSE);
ast_node!(UsageClause, SyntaxKind::USAGE_CLAUSE);
ast_node!(SectionHeader, SyntaxKind::SECTION_HEADER);
ast_node!(Paragraph, SyntaxKind::PARAGRAPH);
ast_node!(ParagraphName, SyntaxKind::PARAGRAPH_NAME);
ast_node!(OpenStatement, SyntaxKind::OPEN_STATEMENT);
ast_node!(CloseStatement, SyntaxKind::CLOSE_STATEMENT);
ast_node!(MoveStatement, SyntaxKind::MOVE_STATEMENT);
ast_node!(PerformStatement, SyntaxKind::PERFORM_STATEMENT);
ast_node!(GoToStatement, SyntaxKind::GO_TO_STATEMENT);
ast_node!(IfStatement, SyntaxKind::IF_STATEMENT);
ast_node!(ElseClause, SyntaxKind::ELSE_CLAUSE);
ast_node!(AddStatement, SyntaxKind::ADD_STATEMENT);
ast_node!(MultiplyStatement, SyntaxKind::MULTIPLY_STATEMENT);
ast_node!(OnSizeErrorClause, SyntaxKind::ON_SIZE_ERROR_CLAUSE);
ast_node!(NotOnSizeErrorClause, SyntaxKind::NOT_ON_SIZE_ERROR_CLAUSE);
ast_node!(WriteStatement, SyntaxKind::WRITE_STATEMENT);
ast_node!(AdvancingClause, SyntaxKind::ADVANCING_CLAUSE);
ast_node!(ExitStatement, SyntaxKind::EXIT_STATEMENT);
ast_node!(Condition, SyntaxKind::CONDITION);

impl SourceFile {
    pub fn program(&self) -> Option<ProgramDefinition> {
        self.syntax.children().find_map(ProgramDefinition::cast)
    }
}

impl ProgramDefinition {
    pub fn identification_division(&self) -> Option<IdentificationDivision> {
        self.syntax
            .children()
            .find_map(IdentificationDivision::cast)
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
